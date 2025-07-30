## Functionality
- [x] Make a new order
- [] Track existing trades
- [] Modify existing trades with new order
- [] Track Pricing
- [] Get Performance information
- [] Filtering parameters for the above

## Architecture
- [] Compile for both Windows and Linux
### Different Application Types: cli, gui
- [] cli
- [] gui
- [] Separate Strategy section so that users can define their own?


# TODO:  add following packages:
1. Color-eyre
2. iRust
3. Bacon
4. Tracing
5. SQLx
6. Clap
    // TODO
    // Implement args: make request, check response,
    // Implement query for historical data
    // Implement query for subscriptions
    // Implement query for order creation, and tracking
    // Try implement using Router
    //
    //Determine if we are running a tui, gui or cli
    // Running a cli only for now

    // Check ENV variables to determine the interface and platform to be used.
    // TODO: Check which platform is going to be used on load
    run_cli();
    run_tui();
    // run_gui();


  How to add filter parameters to getting all open trades

  Desired use case is to be able to monitor trades and prices so that they can be closed at specific prices, and the SL moved to a different point


