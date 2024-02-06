import { FSComponent, DisplayComponent, VNode } from '@microsoft/msfs-sdk';

export class Example extends DisplayComponent<{}> {
    render(): VNode | null {
        return (
            <>
                <text>HelloWorld!</text>
            </>);
    }
}