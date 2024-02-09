// Copyright (c) 2021-2023 FlyByWire Simulations
//
// SPDX-License-Identifier: GPL-3.0
import React from 'react';
import ReactDOM from 'react-dom';
import { getRenderTarget } from '../fbw/src';
export const render = (Slot: React.ReactElement) => {
    const doRender = () => {       

        ReactDOM.render(Slot, getRenderTarget());
    };

    if (process.env.VITE_BUILD) {
        window.addEventListener('AceInitialized', () => doRender());
    } else {
        doRender();
    }
};
