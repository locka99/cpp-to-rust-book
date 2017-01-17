# Foreword

**BEGIN DRAFT BOOK DISCLAIMER **

* Some of the samples will not compile or may not have been syntax checked
* C and Rust code snippets are not distinguished very well yet (styling)
* Some of the text makes uncited assertions of fact
* Some of the text is marked TODO
* Some of the topics that should be covered are brushed over, given undue weight or omitted entirely
* Some of the text probably makes no sense or repeats itself

WITH ALL THAT IN MIND, read on!

**END DRAFT BOOK DISCLAIMER**

Think of all the software that needs to be reliable in this world. Software that can ill afford downtime or crashes. Software that is mission critical and must not or should not fail.

* Operating system services and daemons
* Internet of things devices
* Industrial control software
* Medical devices, imagery etc.
* High availability servers / databases / cloud storage etc.
* Avionics, telemetry, rocketry, drones etc.

All this code that has to run as efficiently and reliably as possible with the minimal of errors. It also has to be predictable without sudden freezes or mystery-memory behavior due to garbage collection.

C and C++ has the speed angle covered but is hard to make reliable. A language like Java would have the reliability angle covered but is hard to make performant.

What we want is something which runs as fast as C or C++ but has the reliability that goes with it. And that is what Rust is about. It compiles into binary executables or libraries just like C or C++ and can even be used to produce dynamic libraries that can be consumed by other code bodies.
