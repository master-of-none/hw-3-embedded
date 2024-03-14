# rgbcal: RGB LED calibration tool
Bart Massey 2024
Added further code and submitted by: Shrikrishna Bhat

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## RGB Value
R - 15 <br>
G - 4 <br>
B - 6 <br>

## Methodology
1. I had to add code for blue LED and Green LED.
2. First I added a function ```initial_state``` such that the even before the code starts running and we press the button a or button b, the rgb starts with color blue or green.
3. Next was adding 2 functions ```change_frame_rate``` and ```change_led``` such that when nothing is pressed only frame rate changes and when buttons are pressed, depending on the button it's R or G or B.
4. Change led function code was already implemented and i just made a different function such that it's much easier to read the code and efficiently change the color of LED based on the requirement.
5. Changing the frame rate required a little careful evaluation.
Here same as the change_led, I used setter and getters concept to make this functioanlilty live. Since the frame rate should be between 10 and 160, I multiplied the level by 10 and then added 10 each time the knob was turned to change the frames. Then I called the setter to set the frames.
6. In the main I added codes for getting and setting th frame_rate. THis setter and getter is same as the setter and getter for RGB.
7. In the RGB file I had to get the frames such that frame rate is sent to tick_time such that time interval is calculated for each frames to be displayed. Hence when frames is 0 the LED starts blinking.
8. Knob file does not have any changes because it is just changing the knob.

## Interesting things observed
1. In RGB when R was set to 15 and G is set between 4-6 and B is set between 6-9, it just looked that it was different shades of white. I had a converstaion with my friends showing the LED that these are different shades of white and they were convinced that these were the same white color.
2. As the professor mentioned, I checked the CPU usage and when frames were high, there was indeed some additional CPU usage.

## Sources
- Majority of the code was just refactoring the given code and setters and getters for frames were just done using the same code for RGB.
