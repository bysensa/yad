use crate::app::event::MacObservedEvent;
use cacao::foundation::{id, nil, NSString};
use cacao::objc::declare::ClassDecl;
use cacao::objc::runtime::{Class, Object, Sel};
use cacao::objc::{class, msg_send, sel, sel_impl, Message};
use objc_foundation::{INSObject, NSObject};
use objc_id::Id;
use std::sync::Once;

extern "C" {
    static NSWorkspaceDidActivateApplicationNotification: id;
    static NSWorkspaceApplicationKey: id;
}

static APPLICATION_ACTIVATION_WATCHER_REGISTER_CLASS: Once = Once::new();

pub struct MacEventObserver(Id<MacEventObserverImpl>);

impl MacEventObserver {
    pub fn new() -> Self {
        MacEventObserver(MacEventObserverImpl::new())
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
        self.dispatch_current_active_application();
        self.setup_application_activated_observer();
        self.setup_lock_observer();
        self.setup_unlock_observer();
    }

    pub fn stop(&mut self) {
        self.dispatch_current_active_application();
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

    fn dispatch_current_active_application(&self) {
        unsafe {
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let current_application: id = msg_send![workspace, frontmostApplication];
            let app_bundle: id = msg_send![current_application, bundleIdentifier];
            let app_bundle = NSString::retain(app_bundle).to_string();

            let localized_name: id = msg_send![current_application, localizedName];
            let localized_name = NSString::retain(localized_name).to_string();

            let pid: i64 = msg_send![current_application, processIdentifier];

            let event = MacObservedEvent::ApplicationActivated(
                pid,
                app_bundle,
                localized_name,
                chrono::Utc::now(),
            );
            event.dispatch();
        }
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
            let notification_name = NSString::new("com.apple.screenIsLocked");
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
            let notification_name = NSString::new("com.apple.screenIsUnlocked");
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
            let app_bundle = NSString::retain(app_bundle).to_string();

            let localized_name: id = msg_send![app, localizedName];
            let localized_name = NSString::retain(localized_name).to_string();

            let pid: i64 = msg_send![app, processIdentifier];

            let event = MacObservedEvent::ApplicationActivated(
                pid,
                app_bundle,
                localized_name,
                chrono::Utc::now(),
            );
            event.dispatch();
        }
    }

    extern "C" fn screen_lock(_: &mut Object, _: Sel, _notification: id) {
        MacObservedEvent::ScreenLocked.dispatch();
    }

    extern "C" fn screen_unlock(_: &mut Object, _: Sel, _notification: id) {
        MacObservedEvent::ScreenUnlocked.dispatch();
    }
}
