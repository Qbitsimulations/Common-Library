/// <reference types="@microsoft/msfs-types/pages/vcockpit/instruments/shared/baseinstrument" />
/// <reference types="@microsoft/msfs-types/pages/vcockpit/core/vcockpit" />

/* global registerInstrument */

import { FSComponent } from '@microsoft/msfs-sdk';
import { Example } from './Example.js';

import './styles.css';

class ExampleInstrument extends BaseInstrument {
    get templateID(): string {
        return 'Q737_Example';
    }

    connectedCallback() {
        super.connectedCallback();

        const root = document.getElementById('MSFS_REACT_MOUNT');

        if (root) {
            root.innerHTML = '';

            FSComponent.render(<Example />, root);
        }
    }
}

registerInstrument('q737-example', ExampleInstrument);