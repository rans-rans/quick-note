import { get, type Writable, writable } from "svelte/store";

export interface HistoryOptions {
    limit?: number; // Max number of undo steps
}

export function createHistoryStore<T>(
    initialValue: T,
    options: HistoryOptions = { limit: 50 },
) {
    const state = writable<T>(initialValue);
    let undoStack: T[] = [];
    let redoStack: T[] = [];

    return {
        subscribe: state.subscribe,

        // Add a new snapshot
        push: (newValue: T) => {
            const current = get(state);
            // Only push if the value has actually changed
            if (JSON.stringify(current) === JSON.stringify(newValue)) return;

            undoStack.push(structuredClone(current));
            if (undoStack.length > (options.limit || 50)) undoStack.shift();

            redoStack = []; // Reset redo on new action
            state.set(newValue);
        },

        state: () => get(state),

        undo: () => {
            if (undoStack.length === 0) return;
            const current = get(state);
            const previous = undoStack.pop()!;

            redoStack.push(structuredClone(current));
            state.set(previous);
        },

        redo: () => {
            if (redoStack.length === 0) return;
            const current = get(state);
            const next = redoStack.pop()!;

            undoStack.push(structuredClone(current));
            state.set(next);
        },

        reset: (value: T) => {
            undoStack = [];
            redoStack = [];
            state.set(value);
        },
    };
}
