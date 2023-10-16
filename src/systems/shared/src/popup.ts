/*
Copyright (c) 2023 FlyByWire Simulations

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

export type Popup = {
    __Type: string;
    buttons: NotificationButton[];
    style: string;
    displayGlobalPopup: boolean;
    contentData: string;
    contentUrl: string;
    contentTemplate: string;
    id: string;
    title: string;
    time: number;
}
export class PopUpDialog {
    params: Popup;

    popupListener: any;

    constructor() {
        const title = 'QBT GENERIC POPUP';
        const time = new Date().getTime();
        this.popupListener = undefined;
        this.params = {
            __Type: 'SNotificationParams',
            buttons: [new NotificationButton('TT:MENU.YES', `QBT_POP_${title}_${time}_YES`), new NotificationButton('TT:MENU.NO', `QBT_POP_${title}_${time}_NO`)],
            style: 'normal',
            displayGlobalPopup: true,
            contentData: 'Default Message',
            contentUrl: '', // i.e. "/templates/Controls/PopUp_EditPreset/PopUp_EditPreset.html";
            contentTemplate: '', // i.e. "popup-edit-preset";
            id: `${title}_${time}`,
            title,
            time,
        };
    }

    /**
     * Pass Popup display data to Coherent
     * @param params
     */
    /* eslint-disable no-underscore-dangle */
    _showPopUp(params: any = {}): void {
        Coherent.trigger('SHOW_POP_UP', params);
    }

    /**
     * Show popup with given or already initiated parameters
     * @param {string} title Title for popup - will show in menu bar
     * @param {string} message Popup message
     * @param {string} style Style/Type of popup. Valid types are small|normal|big|big-help
     * @param {function} callbackYes Callback function -> YES button is clicked.
     * @param {function} callbackNo Callback function -> NO button is clicked.
     */
    showPopUp(title: string, message: string, style: 'small'| 'normal'| 'big'| 'big-help', callbackYes: () => void, callbackNo: () => void): void {
        if (title) {
            this.params.title = title;
        }
        if (message) {
            this.params.contentData = message;
        }
        if (style) {
            this.params.style = style;
        }
        if (callbackYes) {
            const yes = (typeof callbackYes === 'function') ? callbackYes : () => callbackYes;
            Coherent.on(`QBT_POP_${this.params.id}_YES`, () => {
                Coherent.off(`QBT_POP_${this.params.id}_YES`, null, null);
                yes();
            });
        }
        if (callbackNo) {
            const no = (typeof callbackNo === 'function') ? callbackNo : () => callbackNo;
            Coherent.on(`QBT_POP_${this.params.id}_NO`, () => {
                Coherent.off(`QBT_POP_${this.params.id}_NO`, null, null);
                no();
            });
        }

        if (!this.popupListener) {
            this.popupListener = RegisterViewListener('JS_LISTENER_POPUP', this._showPopUp.bind(null, this.params));
        } else {
            this._showPopUp(this.params);
        }
    }

    /**
     * Show information with given or already initiated parameters
     * @param {string} title Title for popup - will show in menu bar
     * @param {string} message Popup message
     * @param {string} style Style/Type of popup. Valid types are small|normal|big|big-help
     * @param {function} callback Callback function -> OK button is clicked.
     */
    showInformation(title: string, message: string, style: 'small'| 'normal'| 'big'| 'big-help', callback: () => void): void {
        if (title) {
            this.params.title = title;
        }
        if (message) {
            this.params.contentData = message;
        }
        if (style) {
            this.params.style = style;
        }
        if (callback) {
            const yes = (typeof callback === 'function') ? callback : () => callback;
            Coherent.on(`QBT_POP_${this.params.id}_YES`, () => {
                Coherent.off(`QBT_POP_${this.params.id}_YES`, null, null);
                yes();
            });
        }
        this.params.buttons = [new NotificationButton('TT:MENU.OK', `QBT_POP_${this.params.id}_YES`)];

        if (!this.popupListener) {
            this.popupListener = RegisterViewListener('JS_LISTENER_POPUP', this._showPopUp.bind(null, this.params));
        } else {
            this._showPopUp(this.params);
        }
    }
}