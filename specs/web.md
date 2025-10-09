# Web Application

> Web API and frontend specification for Mano Machine simulator

## Overview

A WASM-based web interface for assembling and executing Mano assembly programs

## API Endpoints

_To be documented_

## Features

_To be documented_

## Implementation Details

- Should be runnable either from the command line and then viewable from a browser, or served as a single-page-app from 
  a static web server.

### Layout

- 
- Consists of a 4-pane design in 2 groups with a bottom bar and a title bar.
- The title bar should contain "MANO MACHINE SIM" to the left, and a github icon/link to the right.
- The title bar should not be in a box.
- The layout should be responsive and shouldn't be too wide on large screens.
- One large pane on the left in its own group (input group), and 3 smaller panes on the right (output group).
- The groups should be of the same height i.e., panes occupy all available space within the group.
- The 2 groups should be split 50-50.
- The input group is for program input
- The output group's arrangement depends on the current mode.
- The output group is arranged as 2 horizontally adjacent panes in the top half, and a pane in the bottom half.
- The top 2 panes and the bottom pane should be split 60-40 vertically
- The output group's top left pane is for messages, the top right pane for assembly output, and bottom pane for state.
- The messages and assembly panes should be arranged in a 60-40 split
- The input pane should behave similarly to a text editor with line numbers on the left.
- The assembly pane should have line numbers on the left.
- The assembly pane should contain three columns: line "LIN", value in hex "HEX", and value in decimal "DEC"
- The input, message, and assembly panes should be scrollable.
- The message and assembly panes should scroll automatically as content is added 
- The state pane should contain the current memory state as a hex-dump-style grid, and the contents of "MachineState".
- The cpu and memory states should be arranged horizontally to the left and right, respectively.
- The memory and cpu states should be "permanent" in that clearing the state merely zeroes out the values.
- The memory state grid should be arranged in a 4x8 grid.
- The bottom bar should contain buttons for "Assemble", "Run", and "Reset", and a toggle for debug mode.
- The bottom bar should be "floating" with a margin similar to that between panes.


### Design

- Uses a colour scheme similar to solarised dark.
- The interface should be retro-inspired
- The input pane's editor should use some rudimentary colour coding, in line with the colour scheme
- On the right side of the message pane's title bar, there should be a legend of message type colours.
- Each message type should be indicated by a coloured circle followed by the name of the message type.
- The debug indicator should be within the state pane's title bar and shouldn't be in its own box.
- The debug toggle should be a retro-industrial-style button with a "pressed down" state
- There should be some interesting/fun "warning light" style "Debug Mode" indicator during debug mode, without being obtrusive.
- There should be some text at the bottom middle of the screen "COPYRIGHT HSM SYSTEMS 1978"

### Functionality

- All message output should be displayed in the output pane.
- Info and error messages are printed to the messages pane as they're received from Machine.
- Message types should be coloured and prefixed with a coloured circle.
- The "Assemble" button takes the input pane's contents and passes it to prime().
- The assembled program is then displayed in the assembly pane.
- The "Run" button runs tick() in a loop until the machine halts or there's an error in messages.
- The debug button changes the "Run" button to a "Step" button, with red text.
- The "Step" button performs one tick().
- If debug mode is active, print debug messages.
- After exiting the loop, the machine state should be requested from Machine and displayed in the output group.
- The "Reset" button should reset/clear the output group's panes and the machine's state.
