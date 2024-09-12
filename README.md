# <p align="center">GameMan</p>
> The GameBoy emulator that does not play around™

# Screesnhots
<img src="https://github.com/user-attachments/assets/958ef03c-ba16-4add-8f68-de221f90a9cf" width="200">
<img src="https://github.com/user-attachments/assets/853b1c09-2759-46da-b7c3-eb0b7c850cdf" width="200">
<img src="https://github.com/user-attachments/assets/28e64b1a-b5e9-4043-ad83-e8bb6b6f9e62" width="200">

# What can the emulator actually run?
I am at a *very* early stage, this emulator will **not** run *any* game.  
Here's a list of things I need to implement

- [ ] CPU
    - [x] Barebones CPU for testing graphics
    - [x] All opcodes implemented (i just need the STOP one but it's not a priority)
    - [ ] All interrupts implemented
- [x] GPU
    - [x] Barebones and graphicless implementation
    - [x] Background rendering
    - [x] Window rendering
    - [x] Sprite rendering
- [ ] Input Handling
- [ ] GameBoy Color support
- [ ] SPU (Sound), I have yet to research this

Here's a list of roms and games that work on the emulator
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2)
- The panda rom, couldn't find the link

# What makes this emulator different?
- It's **embeddable**, you can embed this into any UI framework, gtk, iced etc. This is not actually an emulator per se, it's the "backend", the "frontend" is up to you to implement, altough I will probably make one myself. As of now, you can try `cargo run --example debugger -- rom.gb` to get a primitive debugger, and `cargo run --example screen -- rom.gb` to see the actual gameboy screen.
- It will *probably* be **embedded** as well in the future, meaning you will be able to run this on a microcontroller
- Focused on **clean code**, this emulator's focus is neither speed or accuracy, if I delivered on this front is up to you, feel free to open an issue or a pull request if you think something can be improved
