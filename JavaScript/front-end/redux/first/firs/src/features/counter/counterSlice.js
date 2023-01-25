import { createSlice } from "@reduxjs/toolkit";

export const counterSlice = createSlice({
  name: 'counter',
  initialState: {
    value: 0
  },
  reducers: {
    /** Increases the state's value by a single number
     *  @param state: The current state of the counterSlice variable
     *  @returns None
     */
    increment: state => {
      state.value += 1;
    },

    /** Decreases the state's value by a single number
     *  @param state: The current state of the counterSlice variable
     *  @returns None
     */
    decrement: state => {
      state.value -= 1;
    },

    /** Modifies the state's value by the given number
     *  @param state: The current state of the counterSlice variable
     *  @param value: The provided value to add to the state
     *  @returns None
     */
    incrementByAmount: (state, value) => {
      state.value += value
    }
  }
});

export const {increment, decrement, incrementByAmount} = counterSlice.actions;
export default counterSlice.reducer;
