# Simple Iced-Widget Timeline lib with example.

This is a simple lib to allow constructing and displaying a timeline **using a
moving window**.  

The aim is twofold:
* have way to display timelines in Iced.
* do it efficiently/economically (both in memory and CPU) by *limiting
calculations to visible elements of the timeline.*


The implementation is fairly generic:
 * allowing the use of any `iced_native::Widget` as the
'Timeline display area' by letting the user implement the initialisation
function: [init_widget](/src/lib.rs#L53).
 * allowing the user to specify how to display each timeline element by letting
 the user implement the [widget_add](/src/lib.rs#L58) function.
 * allowing the Time and The Values to be any type respecting a minimal set of
 traits.

 ### Remarks:
 * This is not a Widget but a Trait.
 * There is no overflow behaviour. If an event does not fit entirely in the
 window it is ignored.

 ## Example:
 This repository contains an example [](examples/text_widget_timeline.rs).  
 This example builds a timeline from 10_000 events where the values to display,
 u32 integers, are displayed as `iced::Text` widgets at the correct position and
 with a width corresponding to the event duration.

 You can try it out with:  
```bash
cargo run --example text_widget_timeline
```

 ## To do / Contributing :
 * figuring out how to use a Canvas as the 'Timeline display area' and what
 changes to make to the [widget_add](/src/lib.rs#L58) function signature and
 calls to permit proper drawing.
 * can the set of necessary traits be even smaller, notably the Copy trait for
 the Values?
 * improve the performance hit for very large numbers (>>10_000) due to
 iterating and filtering the entire timeline in an log(n) lines [56 and 57](https://github.com/GTimothy/iced_timeline/blob/660a072ea994839839a6ea63bb80422064388408/src/lib.rs#L66)
