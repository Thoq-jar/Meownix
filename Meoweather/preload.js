const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electron', {
    requestLocationAccess: () => {
        navigator.geolocation.getCurrentPosition(
            (position) => {
                window.postMessage({
                    type: 'location',
                    payload: position.coords
                }, '*');
            },
            (error) => {
                console.error('Error getting location:', error);
                window.postMessage({
                    type: 'location-error',
                    payload: error.message
                }, '*');
            }
        );
    }
});
