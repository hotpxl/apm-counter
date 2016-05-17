# APM counter

X extension to calculate APM (actions per minute).

There are two versions right now. One is in JavaScript and the other is in Rust. Both read data from `xinput`. Some form of hard coding is required. Currently I use it in a somewhat stable environment, but to make it universal, at least device selection needs to be heuristic.

The JavaScript version displays a progress bar to indicate current APM (moving average). The Rust version uses InfluxDB to log all the data and you might want to use other tools to display the data.
