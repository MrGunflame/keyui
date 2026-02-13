import { writable, type Writable } from "svelte/store";

// A many-to-many reactive signal.
//
// Call `subscribe` to receive an object that will track this signal.
// Call `notify` to signal all active tracker.
//
// Exmaple:
// ```
// let signal = new ReactiveSignal();
// let waker = signal.subscribe();
// $effect(() => {
//     // Track the signal in this effect, causing it to rerun whenever notify is called.
//     $waker;
// })
//
// function onevent() {
//     signal.notify();
// }
// ```
export class ReactiveSignal {
    store: Writable<number>;

    constructor() {
        this.store = writable(0);
    }

    subscribe(): Writable<number> {
        return this.store;
    }

    notify() {
        console.debug("ReactiveSignal::notify");
        this.store.update((n) => n + 1);
    }
}
