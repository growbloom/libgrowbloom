# GrowBloom's temperature

Temperature is a GrowBloom tool to monitor the temperature. It is part of the
GrowBloom suite.

## What is GrowBloom?

GrowBloom is a suite of softwares that help you to manage your environment.
To make something growing you need to manage the temperature, light and so on,
but it's tedious to keep an eye on every sensor.
GrowBloom aims to provide a complete suite to monitor all these items, but also
to manage them. What if using a web portal you could set some triggers like
"if humidity lower that 60% then swith the humidifier on"?

Well, as of writing these lines it is only a dream. But who knows, some day
when you'll read these lines it might be real.

## What is Temperature?

Temperature is a monitoring daemon written in [Rust][rust].
Its work is to communicate with the temperature sensor and report the datas.
The datas could be reported into a database, sent via HTTPS or even written
into a file.

In our dream of triggers, a web portal could call a trigger when Temperature
reports a value.






[rust]: https://www.rust-lang.org/