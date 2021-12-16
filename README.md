# Examples for the Rust Meetup Linz talk

See the subfolders described below for the individual examples.

## unsafe_basics
The examples in the project [unsafe_basics](unsafe_basics) showcase some of the dangers of `unsafe` and try to scare you into using it wisely.
They also show a case of missing error handling and how a safe wrapper around the unsafe code **and the error handler** could have prevented problems there.

## blink
Simple code in [blink](blink) used as a motivating example to write some code for the Pi.
This example simply allows an LED connected to Pin 23 to blink.
We can inspect the used library to get first clues of how to implement this ourselves.

## pi_mem
After learning how the Pi allows us to control GPIO pins in [blink](blink), 
we can do so ourselves as shown in [pi_mem](pi_mem). We use the memory layout as provided in the processor manual.
Please keep in mind, that this only a simple example and the code you find in there wastes several opportunities to make the code safer.

## bare_blink
We use the information gathered up to this point in [bare_blink](bare_blink) to write our kernel that runs without any OS. 
We re-use the code from [pi_mem](pi_mem) to toggle the LED, simply wrap the code in a struct for easier usage.

## hello_stm32
Changing platforms, in [hello_stm32](hello_stm32) we have a quick look at how things are simpler if there are hardware support crates available.
This code also simply allows an LED to blink.

## stm32_btn
Finally, in [stm32_btn](stm32_btn), we take a look of a combination of Rusts security features and low-level hardware programming.
In particular, we take a look at how a special Mutex ensures access to shared mutable state can not be disturbed by an interrupt.
