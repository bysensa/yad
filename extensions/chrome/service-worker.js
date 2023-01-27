let activeTabId, lastUrl, lastTitle;

function getTabInfo(tabId) {
    chrome.tabs.get(tabId, function(tab) {
        if(lastUrl != tab.url || lastTitle != tab.title) {
            console.log(lastUrl = tab.url, lastTitle = tab.title);
            // sendTab();
        }
    });
}

chrome.tabs.onActivated.addListener(function(activeInfo) {
    getTabInfo(activeTabId = activeInfo.tabId);
});

chrome.tabs.onUpdated.addListener(function(tabId, changeInfo, tab) {
    if(activeTabId == tabId) {
        getTabInfo(tabId);
    }
});

//
// chrome.runtime.onInstalled.addListener(async () => {
//     console.log(await getCurrentTab());
//     chrome.notifications.create('NOTFICATION_ID', {
//         type: 'basic',
//         iconUrl: 'path',
//         title: 'notification title',
//         message: 'notification message',
//         priority: 2
//     });
// });
//
function sendTab() {
    fetch('http://0.0.0.0:8000', {
        method: 'post',
        body: "{}"
    }).then(function(r) {
        return r.json();
    }).then(function(data) {
        console.log(data);
    });
}
// //
// // function handleActivated(activeInfo) {
// //     console.log(`Tab ${activeInfo.tabId} was activated`);
// //     chrome.notifications.create('NOTFICATION_ID', {
// //         type: 'basic',
// //         iconUrl: 'path',
// //         title: 'notification title',
// //         message: 'notification message',
// //         priority: 2
// //     })
// // }
//
// browser.tabs.onActivated.addListener(() => {
//     chrome.notifications.create('NOTFICATION_ID', {
//         type: 'basic',
//         iconUrl: 'path',
//         title: 'notification title',
//         message: 'notification message',
//         priority: 2
//     })
// });
//
async function getCurrentTab() {
    let queryOptions = { active: true, lastFocusedWindow: true };
    // `tab` will either be a `tabs.Tab` instance or `undefined`.
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}