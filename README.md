# makepad-widgets-sample

This sample app can be modified to run in 3 different "modes":

1. Widgets mode - shows a few basic widgets such as buttons, button actions, labels, text input, and drop downs
1. Layout mode - shows buttons in different locations on screen using row, column type layout
1. Slider mode - shows a "slideshow" view which includes both above modes.

This project uses the makepad development branch of "rik"
It assumes that the project is installed in a parallel directory as makepad and that makepad is accessible via '../makepad'
(update in Cargo.toml if otherwise)

## Build & Run Instructions

Follow step 1 commands below for initial setup of the Makepad build and run environment.
After step 2, you may choose any one or more of the platforms you're interested in building for.

## 1. Setup Makepad

Replace `projects` with your own directory name.

```bash
cd ~/projects
```

### Clone the Makepad repository

```bash
git clone git@github.com:makepad/makepad.git
```

### Change to latest 'rik' branch (Optional)

```bash
cd ~/projects/makepad
git branch rik
```

### Install makepad subcommand for cargo

```bash
cd ~/projects/makepad
cargo install --path ./tools/cargo_makepad
```

## 2. Get This Project

### Clone the makepad_widgets_sample repo

```bash
cd ~/projects
git clone https://github.com/project-robius/makepad_widgets_sample
```

## 3. MacOS / PC

Running on Desktop is the quickest way to try out an example app.

```bash
cd ~/projects/makepad_widgets_sample
cargo run
```

or

```bash
cd ~/projects/makepad_widgets_sample
cargo run -p makepad_widgets_sample --release
```

And there should be a desktop application window now running (may need to click on the icon on MacOS's Dock to show it)

## 4. Android Build

### Install Android toolchain (First time)

```bash
rustup toolchain install nightly
cargo makepad android install-toolchain
```

### Install app on Android devivce or Android emulator

Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a device connected properly

```bash
cd ~/projects makepad_widgets_sample
cargo makepad android run -p makepad_widgets_sample --release
```

## 5. iOS Setup & Install

### Install IOS toolchain (First time)

```bash
rustup toolchain install nightly
cargo makepad ios install-toolchain
```

### Install app on Apple devivce or iOS simulator

### iOS Setup

For iOS, the process is slightly more complicated. The steps involved are:

1. Enable your iPhone's Developer Mode, please see instructions here: [Enable Developer Mode](https://www.delasign.com/blog/how-to-turn-on-developer-mode-on-an-iphone/)
1. Setup an Apple Developer account
1. Setup an empty skeleton project in XCode
    1. File -> New -> Project to create a new "App"
    1. Set the Product Name as **`WidgetsSample`**  (used in --org later)
    1. Set the Organization Identifier to a value of your choice, for this example we will use **`rs.robius`**. (used in --app later)
    1. Setup the Project Signing & Capabilities to select the proper team account 
1. In XCode, Build/Run this project to install and run the app on the simulator and device
1. Once the simulator and device has the "skeleton" app installed and running properly, then it is ready for Makepad to install its application.

### Makepad Install

We will run the `cargo makepad ios` command, similar to Android build above, but there are some 3 to 4 additional parameters that need to be filled in:

`--org-id`

This is the <string> value of the ApplicationIdentifierPrefix <key> in the `**.mobileprovision` file located in the `~/Library/MobileDevice/Provisioning Profiles` directory.
It should be a 10 digit alphanumeric value.

`--org`

First few parts of the organization identifier (which makes up the Bundle Identifier). Usually in the form of **com.somecompany** or **org.someorg**
This is the same value used to setup the initial skeleton app above. For this example:
> `rs.robius`

`--app`

The name of the application or the project. This is the same as the Product Name used to setup the initial skeleton app above. In this case:
> `WidgetsSample`

`--ios-version` (optional)

defaults to 17. Set it to 16 or other values if the device is not running iOS 17.

### Example

For this example, we have the Bundle Identifier of **`rs.robius.WidgetsSample`**

### Install app on IOS simulator

```bash
cd ~/projects/makepad_widgets_sample
cargo makepad ios --org=rs.robius --app=WidgetsSample run-sim -p makepad_widgets_sample --release
```

### Install app on IOS device

```bash
cd ~/projects/makepad_widgets_sample
cargo makepad ios --ios-version=16 --org-id=<ORGIDVALUE> --org=rs.robius --app=WidgetsSample run-device -p makepad_widgets_sample --release
```

## 6. WASM Build

*Coming Soon*
