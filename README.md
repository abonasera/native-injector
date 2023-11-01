# Rust & Java JNI Dynamic Library Injector
![Platform: Windows](https://img.shields.io/badge/Platform-Windows-lightgrey.svg)

## Overview
This project serves as a powerful tool designed to inject native libraries into running JVM processes. Built in Rust, this project enables dynamic and remote JVM method calling through JNI and has advanced features for memory manipulation and process control.

## Usage
This project aims to be a framework for a variety of different uses. Using the command line and a compiled `nativeinjector.jar` either built or downloaded from the releases tab, you can select a running Java VM by PID and inject the payload library into it.

For example, lets say you want to inject into my `Obfuscator` project that can also be found on my GitHub page.

![1](https://i.imgur.com/hXc9RHm.png)

With the program running, we can open a new `cmd` window and run our injector.

![2](https://i.imgur.com/wzNQQrA.png)

Then, we will enter the desired PID.

![3](https://i.imgur.com/0NGQMKe.png)

If we look back to our console running the Obfuscator, we will see a new message.

![4](https://i.imgur.com/SrXmay4.png)

The native injector has successfully called the `println` function remotely on a running process.

While this may not be the most useful implementation, what the payload does is entirely up to you. The Java Native Interface provides a multitude of different functions in Rust, including but not limited to:
* Calling Java functions remotely
* Accessing fields, classes, and objects
* Defining classes dynamically
* Throwing and catch Java exceptions
* Attaching or detaching new threads
* Passing strings back and forth between languages

In essence, you have complete control over a foreign process. In order to modify the injector to your needs, navigate to the `src/main/rust/payload/` folder. Inside of `lib.rs` is an example injection function that prints a message to the target source.

## Building
The Gradle buildscript is set up for automatic building of not only the project but also the `injector` and `payload` source sets. In an IDE environment, the injector and payload DLLs will be built and moved to the `resources` folder automatically before Java starts. When building, they will also automatically be built and compiled into the final jar. There should be no need to use cargo commands in console.

## Additional Information
For more information on Rust JNI, visit the [Rust JNI Documentation](https://docs.rs/jni/0.19.0/jni/).
