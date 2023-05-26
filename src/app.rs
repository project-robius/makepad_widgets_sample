use makepad_widgets::*;

// The live_design macro generates a function that registers a DSL code block with the global
// context object (`Cx`).
//
// DSL code blocks are used in Makepad to facilitate live design. A DSL code block defines
// structured data that describes the styling of the UI. The Makepad runtime automatically
// initializes widgets from their corresponding DSL objects. Moreover, external programs (such
// as a code editor) can notify the Makepad runtime that a DSL code block has been changed, allowing
// the runtime to automatically update the affected widgets.
live_design!{
    import makepad_widgets::button::Button;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::drop_down::DropDown;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;
    import makepad_draw::shader::std::*;

    import makepad_widgets::slides_view::Slide;
    import makepad_widgets::slides_view::SlideChapter;
    import makepad_widgets::slides_view::SlideBody;
    import makepad_widgets::slides_view::SlidesView;

    SCREEN_WIDTH = 1200;
    FONT_SIZE_H2 = 9.5
    
    SPACING_CONTROLS = 7.5;

    SSPACING_0 = 0.0
    SSPACING_1 = 4.0
    SSPACING_2 = (SSPACING_1 * 2)
    SSPACING_3 = (SSPACING_1 * 3)
    SSPACING_4 = (SSPACING_1 * 4)
    
    SPACING_0 = {top: (SSPACING_0), right: (SSPACING_0), bottom: (SSPACING_0), left: (SSPACING_0)}
    SPACING_1 = {top: (SSPACING_1), right: (SSPACING_1), bottom: (SSPACING_1), left: (SSPACING_1)}
    SPACING_2 = {top: (SSPACING_2), right: (SSPACING_2), bottom: (SSPACING_2), left: (SSPACING_2)}
    SPACING_3 = {top: (SSPACING_3), right: (SSPACING_3), bottom: (SSPACING_3), left: (SSPACING_3)}
    SPACING_4 = {top: (SSPACING_4), right: (SSPACING_4), bottom: (SSPACING_4), left: (SSPACING_4)}
    H2_TEXT_BOLD = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
    }
    H2_TEXT_NORMAL = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    COLOR_DOWN_FULL = #000
    
    COLOR_DOWN_0 = #x00000000
    COLOR_UP_0 = #xFFFFFF00
    COLOR_UP_5 = #xFFFFFF66

    ElementBox = <Frame> {
        draw_bg: {color: (COLOR_DOWN_0)}
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, padding: <SPACING_1> {}, spacing: (SSPACING_1)}
    }

    FishDropDown = <DropDown> {
        walk: {width: Fit}
        layout: {padding: {top: (SSPACING_2), right: (SSPACING_4), bottom: (SSPACING_2), left: (SSPACING_2)}}
        
        draw_label: {
            text_style: <H2_TEXT_NORMAL> {},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            (#xFFF8),
                            (#xFFF8),
                            self.focus
                        ),
                        (#xFFFF),
                        self.hover
                    ),
                    (#x000A),
                    self.pressed
                )
            }
        }
        
        popup_menu: {
            menu_item: {
                indent_width: 10.0
                walk: {width: Fill, height: Fit}
                
                layout: {
                    padding: {left: (SSPACING_4), top: (SSPACING_2), bottom: (SSPACING_2), right: (SSPACING_4)},
                }
                
                draw_bg: {
                    color: #x48,
                    color_selected: #x6
                }
            }
        }
        
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                self.get_bg(sdf);
                // triangle
                let c = vec2(self.rect_size.x - 10.0, self.rect_size.y * 0.5)
                let sz = 2.5;
                
                sdf.move_to(c.x - sz, c.y - sz);
                sdf.line_to(c.x + sz, c.y - sz);
                sdf.line_to(c.x, c.y + sz * 0.75);
                sdf.close_path();
                
                sdf.fill(mix(#FFFA, #FFFF, self.hover));
                
                return sdf.result
            }
            
            fn get_bg(self, inout sdf: Sdf2d) {
                sdf.rect(
                    0,
                    0,
                    self.rect_size.x,
                    self.rect_size.y
                )
                sdf.fill((COLOR_UP_0))
            }
        }
    }

    InstrumentDropdown = <ElementBox> {
        layout: {align: {y: 0.5}, padding: <SPACING_0> {}, flow: Right}
        label = <Label> {
            walk: {width: Fit}
            draw_label: {
                color: (COLOR_UP_5)
                text_style: <H2_TEXT_BOLD>{},
            }
        }
        dropdown = <FishDropDown> {
            walk: {margin: {left: (SSPACING_1), right: (SSPACING_1)}}
        }
    }


    WidgetFrame = <Frame>{
        show_bg: true
        // The `layout` property determines how child widgets are laid out within a frame. In
        // this case, child widgets flow downward, with 20 pixels of spacing in between them,
        // and centered horizontally with respect to the entire frame.
        //
        // Because the child widgets flow downward, vertical alignment works somewhat
        // differently. In this case, children are centered vertically with respect to the
        // remainder of the frame after the previous children have been drawn.
        layout: {
            flow: Down,
            spacing: 20,
            align: {
                x: 0.5,
                y: 0.5
            }
        },
        // The `walk` property determines how the frame widget itself is laid out. In this
        // case, the frame widget takes up the entire window.
        walk: {
            width: Fill,
            height: Fill
        },
        draw_bg: {
            // The `fn pixel(self) -> vec4` syntax is used to define a property named `pixel`,
            // the value of which is a shader. We use our own custom DSL to define shaders. It's
            // syntax is *mostly* compatible with GLSL, although there are some differences as
            // well.
            fn pixel(self) -> vec4 {
                // Within a shader, the `self.geom_pos` syntax is used to access the `geom_pos`
                // attribute of the shader. In this case, the `geom_pos` attribute is built in,
                // and ranges from 0 to 1.
                return mix(#7, #5, self.geom_pos.y);
            }
        }
        
        // The `name:` syntax is used to define fields, i.e. properties for which there are
        // corresponding struct fields. In contrast, the `name =` syntax is used to define
        // instance properties, i.e. properties for which there are no corresponding struct
        // fields. Note that fields and instance properties use different namespaces, so you
        // can have both a field and an instance property with the same name.
        //
        // Widgets can hook into the Makepad runtime with custom code and determine for
        // themselves how they want to handle instance properties. In the case of frame widgets,
        // they simply iterate over their instance properties, and use them to instantiate their
        // child widgets.
        
        // A button to increment the counter.
        //
        // The `<Button>` syntax is used to inherit a DSL object from another DSL object. This
        // tells the Makepad runtime our DSL object has the same properties as the DSL object
        // named `Button`, except for the properties defined here below, which override any

        ButtonFrame = <Frame> {     
            walk: {width: Fit, height: Fit}
            layout: {
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                }
            },       
            button1 = <Button> {
                // icon_walk:{margin:{left:10}, width:16,height:Fit}
                draw_label: {
                    text_style:<REGULAR_TEXT>{},
                    color: #f
                },
                label: "Button +"
            }

            label1 = <Label> {
                walk: {width: 100}
                align: {
                    x: 0.3,
                    // y: 1
                }
                draw_label: {
                    text_style:<REGULAR_TEXT>{},
                    color: #f
                },
                label: "Label: 0"
            }

            button2 = <Button> {
                // icon_walk:{margin:{left:10}, width:16,height:Fit}
                draw_label: {
                    text_style:<REGULAR_TEXT>{},
                    color: #f
                },
                label: "Button -"
            }
        }

        InputFrame = <Frame> {
            walk: {width: Fit, height: Fit}
            layout: {
                flow: Right,
                spacing: 10,
                align: {
                    x: 0.5,
                    y: 0.0
                }
            },

            label_input = <Label> {
                walk: {height:30},
                align: {
                    // x: 1.5,
                    y: 1
                }
                draw_label: {
                    text_style:<REGULAR_TEXT>{},
                    color: #f
                },
                label: "Text Input:"
            }

            input_sample = <TextInput> {
                // instance border_width: 2.0,
                // walk: {width:500, height:30},
                draw_bg: {
                    color: #333
                }
                draw_label: {
                    text_style:<REGULAR_TEXT>{},
                    color: #aaaaaa
                }
                text: "Enter Text Here"
            }
        }

        DropDownFrame = <Frame> {
            walk: {width: Fit, height: Fit}
            layout: {
                flow: Right,
                spacing: 10,
                align: {
                    x: 0.5,
                    y: 0.0
                }
            },

            label_dropdown = <Label> {
                walk: {height:30},
                align: {
                    // x: 1.5,
                    y: 1
                }
                draw_label: {
                    // text_style:<REGULAR_TEXT>{},
                    color: #f
                },
                label: "Dropdown:"
            }

            my_dropdown = <InstrumentDropdown> {
                layout: {flow: Down}
                walk: {
                    width: Fit,
                    height: 30,
                    margin: {
                        top: (SPACING_CONTROLS),
                        right: (SPACING_CONTROLS),
                        bottom: (SPACING_CONTROLS),
                        left: 0.0
                    }
                }
                dropdown = {
                    values: [sel1, sel2, sel3, sel4]
                    labels: ["Selection 1", "Selection 2", "Selection 3","Selection 4"]
                }
            }
        }
    }

    LayoutFrame = <Frame>{
        show_bg: true
        // The `layout` property determines how child widgets are laid out within a frame. In
        // this case, child widgets flow downward, with 20 pixels of spacing in between them,
        // and centered horizontally with respect to the entire frame.
        //
        // Because the child widgets flow downward, vertical alignment works somewhat
        // differently. In this case, children are centered vertically with respect to the
        // remainder of the frame after the previous children have been drawn.
        layout: {
            flow: Down,
            // spacing: 20,
            align: {
                x: 0.5,
                y: 0.5
            }
        },
        // The `walk` property determines how the frame widget itself is laid out. In this
        // case, the frame widget takes up the entire window.
        walk: {
            width: Fill,
            height: Fill
        },
        draw_bg: {
            // The `fn pixel(self) -> vec4` syntax is used to define a property named `pixel`,
            // the value of which is a shader. We use our own custom DSL to define shaders. It's
            // syntax is *mostly* compatible with GLSL, although there are some differences as
            // well.
            fn pixel(self) -> vec4 {
                // Within a shader, the `self.geom_pos` syntax  is used to access the `geom_pos`
                // attribute of the shader. In this case, the `geom_pos` attribute is built in,
                // and ranges from 0 to 1.
                return mix(#5, #5, self.geom_pos.y);
            }
        }

        CornerFrame1 = <Frame> {
            layout: {
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.0
                }
            },
            buttonc1 = <Button> {
                walk: {width: 120}
                label: "Button Top"
            }
        }

        CornerFrame2 = <Frame> {
            layout: {
                flow: Right,
                spacing: 20,
                align: {
                    x: 0.0,
                    y: 0.0
                }
            },

            CornerFrame21 = <Frame> {
                layout: {
                    flow: Down,
                    spacing: 20,
                    align: {
                        x: 0.0,
                        y: 0.5
                    }
                },
                buttonc21 = <Button> {
                    walk: {width: 120}
                    label: "Button Left"
                }
            }

            // <WidgetFrame> {}

            CornerFrame22 = <Frame> {
                layout: {
                    flow: Down,
                    spacing: 20,
                    align: {
                        x: 0.5,
                        y: 0.5
                    }
                },
                buttonc22 = <Button> {
                    walk: {width: 120}
                    label: "Button Center"
                }
            }

            CornerFrame23 = <Frame> {
                layout: {
                    flow: Down,
                    spacing: 20,
                    align: {
                        x: 1.0,
                        y: 0.5
                    }
                },
                buttonc23 = <Button> {
                    walk: {width: 120}
                    label: "Button Right"
                }
            }
        }
        CornerFrame3 = <Frame> {
            layout: {
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 1.0
                }
            },
            buttonc3 = <Button> {
                walk: {width: 120}
                label: "Button Bottom"
            }
        }
    }

    // The `{{App}}` syntax is used to inherit a DSL object from a Rust struct. This tells the
    // Makepad runtime that our DSL object corresponds to a Rust struct named `App`. Whenever an
    // instance of `App` is initialized, the Makepad runtime will obtain its initial values from
    // this DSL object.
    App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
 
        // ui=<DesktopWindow>{
        //     <LayoutFrame> {}
        // }

        ui:<DesktopWindow>{
            <WidgetFrame> {}
        }

        // ui=<DesktopWindow>{
        //     show_bg: true
        //     // The `layout` property determines how child widgets are laid out within a frame. In
        //     // this case, child widgets flow downward, with 20 pixels of spacing in between them,
        //     // and centered horizontally with respect to the entire frame.
        //     //
        //     // Because the child widgets flow downward, vertical alignment works somewhat
        //     // differently. In this case, children are centered vertically with respect to the
        //     // remainder of the frame after the previous children have been drawn.
        //     layout: {
        //         flow: Down,
        //         // spacing: 20,
        //         align: {
        //             x: 0.5,
        //             y: 0.5
        //         }
        //     },
        //     // The `walk` property determines how the frame widget itself is laid out. In this
        //     // case, the frame widget takes up the entire window.
        //     walk: {
        //         width: Fill,
        //         height: Fill
        //     },
        //     draw_bg: {
        //         // The `fn pixel(self) -> vec4` syntax is used to define a property named `pixel`,
        //         // the value of which is a shader. We use our own custom DSL to define shaders. It's
        //         // syntax is *mostly* compatible with GLSL, although there are some differences as
        //         // well.
        //         fn pixel(self) -> vec4 {
        //             // Within a shader, the `self.geom_pos` syntax is used to access the `geom_pos`
        //             // attribute of the shader. In this case, the `geom_pos` attribute is built in,
        //             // and ranges from 0 to 1.
        //             return mix(#8, #5, self.geom_pos.y);
        //         }
        //     }
            
        //     // The `name:` syntax is used to define fields, i.e. properties for which there are
        //     // corresponding struct fields. In contrast, the `name =` syntax is used to define
        //     // instance properties, i.e. properties for which there are no corresponding struct
        //     // fields. Note that fields and instance properties use different namespaces, so you
        //     // can have both a field and an instance property with the same name.
        //     //
        //     // Widgets can hook into the Makepad runtime with custom code and determine for
        //     // themselves how they want to handle instance properties. In the case of frame widgets,
        //     // they simply iterate over their instance properties, and use them to instantiate their
        //     // child widgets.
            
        //     // A button to increment the counter.
        //     //
        //     // The `<Button>` syntax is used to inherit a DSL object from another DSL object. This
        //     // tells the Makepad runtime our DSL object has the same properties as the DSL object
        //     // named `Button`, except for the properties defined here below, which override any

        //     ButtonFrame = <Frame> {     
        //         walk: {width: Fit, height: Fit}
        //         layout: {
        //             flow: Down,
        //             spacing: 20,
        //             align: {
        //                 x: 0.5,
        //                 y: 0.5
        //             }
        //         },       
        //         button1 = <Button> {
        //             // icon_walk:{margin:{left:10}, width:16,height:Fit}
        //             label: "Button +"
        //         }

        //         label1 = <Label> {
        //             walk: {width: 100}
        //             align: {
        //                 x: 0.5,
        //                 // y: 1
        //             }
        //             draw_label: {
        //                 color: #f
        //             },
        //             label: "Label: 0"
        //         }
        //         button2 = <Button> {
        //             // icon_walk:{margin:{left:10}, width:16,height:Fit}
        //             label: "Button -"
        //         }
        //     }

        //     InputFrame = <Frame> {
        //         walk: {width: Fit, height: Fit}
        //         layout: {
        //             flow: Right,
        //             spacing: 10,
        //             align: {
        //                 x: 0.5,
        //                 y: 0.0
        //             }
        //         },

        //         label_example = <Label> {
        //             walk: {height:30},
        //             align: {
        //                 // x: 1.5,
        //                 y: 1
        //             }
        //             draw_label: {
        //                 color: #f
        //             },
        //             label: "Text Input:"
        //         }

        //         input_sample = <TextInput> {
        //             // instance border_width: 2.0,
        //             // walk: {width:120, height:30},
        //             draw_bg: {
        //                 color: #333
        //             }
        //             draw_label: {
        //                 text_style:<REGULAR_TEXT>{font_size: (14)},
        //                 color: #aaaaaa
        //             }
        //             text: "Enter Text Here to change the label"
        //         }
        //     }

        //     DropDownFrame = <Frame> {
        //         walk: {width: Fit, height: Fit}
        //         layout: {
        //             flow: Right,
        //             spacing: 10,
        //             align: {
        //                 x: 0.5,
        //                 y: 0.0
        //             }
        //         },

        //         label_dropdown = <Label> {
        //             walk: {height:30},
        //             align: {
        //                 // x: 1.5,
        //                 y: 1
        //             }
        //             draw_label: {
        //                 color: #f
        //             },
        //             label: "Dropdown:"
        //         }

        //         my_dropdown = <InstrumentDropdown> {
        //             layout: {flow: Down}
        //             walk: {
        //                 width: Fit,
        //                 height: 30,
        //                 margin: {
        //                     top: (SPACING_CONTROLS),
        //                     right: (SPACING_CONTROLS),
        //                     bottom: (SPACING_CONTROLS),
        //                     left: 0.0
        //                 }
        //             }
        //             dropdown = {
        //                 values: [sel1, sel2, sel3, sel4]
        //                 labels: ["Selection 1", "Selection 2", "Selection 3","Selection 4"]
        //             }
        //         }
        //     }
        // }

        ui=<DesktopWindow> {
            window: {inner_size: vec2(1280, 1080)},
            pass: {clear_color: #2A}
            block_signal_event: true; 
            <SlidesView> {
                goal_pos: 0.0
                
                <SlideChapter> {
                    title = {label: "MAKEPAD\nWIDGETS"},
                    <SlideBody> {label: "\n"}
                }
                <Slide> {
                    title = {label: ""},
                    <SlideBody> {label: "Widgets"}
                }
                <Slide> {title = {label: ""}, 
                    <Box>{
                        draw_bg: { color: #x2A }
                        walk: { width: (SCREEN_WIDTH) }
                        layout:{ padding: 0.0 }
                        <WidgetFrame> {}
                    }
                }
                <Slide> {
                    title = {label: ""},
                    <SlideBody> {label: "Layout"}
                }
                <Slide> {title = {label: ""}, 
                    <Box>{
                        draw_bg: { color: #x2A }
                        walk: { width: (SCREEN_WIDTH) }
                        layout:{ padding: 0.0 }
                        <LayoutFrame> {}
                    }
                }             
            }
        }
    }
}

// This app_main macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop. On web, this means
// creating an event handler function that the browser event loop can call into.
app_main!(App);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live)]
// This function is used to register any DSL code that you depend on.
// called automatically by the code we generated with the call to the macro `main_app` above.
pub struct App {
    // A chromeless window for our application. Used to contain our frame widget.
    // A frame widget. Used to contain our button and label.
    #[live] ui: WidgetRef,
    // #[live] label_example: LabelRef,

    // The value for our counter.
    //
    // The #[rust] attribute here is used to indicate that this field should *not* be initialized
    // from a DSL object, even when a corresponding property exists.
    #[rust] counter: usize,
    #[rust] sample: String,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl App {
    pub fn data_bind(&mut self, mut db: DataBindingMap) {
        db.bind(id!(my_dropdown), ids!(my_dropdown.dropdown));
    }
}

impl AppMain for App {
    // This function is used to handle any incoming events from the host system. It is called
    // automatically by the code we generated with the call to the macro `main_app` above.
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            // This is a draw event, so create a draw context and use that to draw our application.
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        // Forward the event to the frame. In this case, handle_event returns a list of actions.
        // Actions are similar to events, except that events are always forwarded downward to child
        // widgets, while actions are always returned back upwards to parent widgets.
        let actions = self.ui.handle_widget_event(cx, event);
        
        // Get a reference to our button from the frame, and check if one of the actions returned by
        // the frame was a notification that the button was clicked.
        if self.ui.get_button(id!(button1)).clicked(&actions) {
            // Increment the counter.
            self.counter += 1;
            
            // Get a reference to our label from the frame, update its text, and schedule a redraw
            // for it.
            let label = self.ui.get_label(id!(label1));
            label.set_label(&format!("Label: {}", self.counter));
            label.redraw(cx);
        }

        if self.ui.get_button(id!(button2)).clicked(&actions) {
            // Decrement the counter.
            if self.counter >= 1 {
                self.counter -= 1;
            }
            
            // Get a reference to our label from the frame, update its text, and schedule a redraw
            // for it.
            let label = self.ui.get_label(id!(label1));
            label.set_label(&format!("Label: {}", self.counter));
            label.redraw(cx);
        }

        for widget_action in &actions {
            if let TextInputAction::Return(value) = widget_action.action::<TextInputAction>() {
                if !value.is_empty() {
                    // println!("value={}", value);
                    let label =self.ui.get_label(id!(label_input));
                    label.set_label(&format!("{}",  value));
                    label.redraw(cx);
                    break
                }
            }
        }

        let ui = self.ui.clone();
        let mut drop_db = DataBindingStore::new();
        self.data_bind(drop_db.widgets_to_data(cx, &actions, &ui));
        self.data_bind(drop_db.data_to_widgets(cx, &actions, &ui));

    }
}