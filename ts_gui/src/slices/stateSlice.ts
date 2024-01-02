import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface ICrashOpen {
    state: boolean;
    errorMessage?: string;
}

interface IState {
    crashOpen: ICrashOpen;
}

const initialState: IState = {
    crashOpen: {
        state: false,
        errorMessage: ""
    }
}

export const state = createSlice({
    name: "state",
    initialState,
    reducers: {
        setCrashOpen: (state, action: PayloadAction<ICrashOpen>) => {
            state.crashOpen = action.payload;
        }
    }
});

export const { setCrashOpen } = state.actions
export default state.reducer