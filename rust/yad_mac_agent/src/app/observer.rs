use cacao::appkit::AppDelegate;
use cacao::notification_center::Dispatcher;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use flume::*;
use lazy_static::lazy_static;
use objc::declare::ClassDecl;
use objc::runtime::*;
use objc::*;
use objc_foundation::{INSObject, NSObject};
use objc_id::Id;
use std::ptr;
use std::sync::Once;

pub trait Observer: AppDelegate + Dispatcher<Message = MacObservedEvent> + Sized {}

extern "C" {
    static NSWorkspaceDidActivateApplicationNotification: id;
    static NSWorkspaceApplicationKey: id;
}

lazy_static! {
    static ref EVENTS_CHANNEL: (EventSender, EventReceiver) =
        flume::unbounded::<MacObservedEvent>();
}

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type ApplicationName = String;
pub type BundleId = String;
pub type Pid = i64;
type EventSender = Sender<MacObservedEvent>;
type EventReceiver = Receiver<MacObservedEvent>;

#[derive(Debug)]
pub enum MacObservedEvent {
    ApplicationActivated(Pid, BundleId, ApplicationName, DateTime),
    ScreenLocked,
    ScreenUnlocked,
}

static APPLICATION_ACTIVATION_WATCHER_REGISTER_CLASS: Once = Once::new();

pub struct MacEventObserver(Id<MacEventObserverImpl>);

impl MacEventObserver {
    pub fn new() -> Self {
        MacEventObserver(MacEventObserverImpl::new())
    }

    pub fn receiver() -> EventReceiver {
        EVENTS_CHANNEL.1.clone()
    }

    pub fn start(&mut self) {
        self.0.start();
    }

    pub fn stop(&mut self) {
        self.0.stop();
    }
}

struct MacEventObserverImpl;

unsafe impl Message for MacEventObserverImpl {}

impl INSObject for MacEventObserverImpl {
    fn class() -> &'static Class {
        APPLICATION_ACTIVATION_WATCHER_REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new("MacEventObserverImpl", superclass).unwrap();

            unsafe {
                decl.add_method(
                    sel!(workspace_app_activated:),
                    Self::workspace_app_activated as extern "C" fn(&mut Object, Sel, id),
                );
                decl.add_method(
                    sel!(screen_lock:),
                    Self::screen_lock as extern "C" fn(&mut Object, Sel, id),
                );
                decl.add_method(
                    sel!(screen_unlock:),
                    Self::screen_unlock as extern "C" fn(&mut Object, Sel, id),
                );
            }

            decl.register();
        });
        Class::get("MacEventObserverImpl").unwrap()
    }
}

impl MacEventObserverImpl {
    pub fn start(&mut self) {
        self.setup_application_activated_observer();
        self.setup_lock_observer();
        self.setup_unlock_observer();
    }

    pub fn stop(&mut self) {
        unsafe {
            let inst = &*(self as *const _ as *const Object);
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let notification_center: id = msg_send![workspace, notificationCenter];
            let distributed_notification_center: id =
                msg_send![class!(NSDistributedNotificationCenter), defaultCenter];
            let _: id = msg_send![notification_center, removeObserver: inst];
            let _: id = msg_send![distributed_notification_center, removeObserver: inst];
            let _: id = msg_send![distributed_notification_center, removeObserver: inst];
        };
    }

    fn setup_application_activated_observer(&mut self) {
        unsafe {
            let inst = &mut *(self as *mut _ as *mut Object);
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let notification_center: id = msg_send![workspace, notificationCenter];
            let _: id = msg_send![notification_center, addObserver: inst as *mut Object
										    	selector:sel!(workspace_app_activated:)
													name: NSWorkspaceDidActivateApplicationNotification
												  object: nil];
        };
    }

    fn setup_lock_observer(&mut self) {
        unsafe {
            let inst = &mut *(self as *mut _ as *mut Object);
            let distributed_notification_center: id =
                msg_send![class!(NSDistributedNotificationCenter), defaultCenter];
            let notification_name = NSString::alloc(nil).init_str("com.apple.screenIsLocked");
            let _: id = msg_send![distributed_notification_center, addObserver: inst as *mut Object
										    	selector:sel!(screen_lock:)
													name: notification_name
												  object: nil];
        }
    }

    fn setup_unlock_observer(&mut self) {
        unsafe {
            let inst = &mut *(self as *mut _ as *mut Object);
            let distributed_notification_center: id =
                msg_send![class!(NSDistributedNotificationCenter), defaultCenter];
            let notification_name = NSString::alloc(nil).init_str("com.apple.screenIsUnlocked");
            let _: id = msg_send![distributed_notification_center, addObserver: inst as *mut Object
										    	selector:sel!(screen_unlock:)
													name: notification_name
												  object: nil];
        }
    }

    extern "C" fn workspace_app_activated(_: &mut Object, _: Sel, notification: id) {
        unsafe {
            let dict: id = msg_send![notification, userInfo];
            let app: id = msg_send![dict, objectForKey: NSWorkspaceApplicationKey];

            let app_bundle: id = msg_send![app, bundleIdentifier];
            let app_bundle = ns_string_to_string(app_bundle);

            let localized_name: id = msg_send![app, localizedName];
            let localized_name = ns_string_to_string(localized_name);

            let pid: i64 = msg_send![app, processIdentifier];

            let event = MacObservedEvent::ApplicationActivated(
                pid,
                app_bundle,
                localized_name,
                chrono::Utc::now(),
            );
            let sender = EVENTS_CHANNEL.0.clone();
            // let _ = sender.send(event);

            cacao::appkit::App::<dyn Observer<Message = MacObservedEvent>, MacObservedEvent>::dispatch_main(event)
        }
    }

    extern "C" fn screen_lock(_: &mut Object, _: Sel, _notification: id) {
        let event = MacObservedEvent::ScreenLocked;
        let sender = EVENTS_CHANNEL.0.clone();
        let _ = sender.send(event);
    }

    extern "C" fn screen_unlock(_: &mut Object, _: Sel, _notification: id) {
        let event = MacObservedEvent::ScreenUnlocked;
        let sender = EVENTS_CHANNEL.0.clone();
        let _ = sender.send(event);
    }
}

fn ns_string_to_string(ns_string: id) -> String {
    unsafe {
        let len: usize = msg_send![ns_string, length];

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        bytes.set_len(len);
        ptr::copy_nonoverlapping(ns_string.UTF8String() as *const u8, bytes.as_mut_ptr(), len);

        String::from_utf8(bytes).unwrap()
    }
}
