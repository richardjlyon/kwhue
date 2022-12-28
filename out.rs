#![feature(prelude_import)]
//! Library for Hue lights.
//!
//! This crate provides a library andcommand line application for contolling hue
//! lights.
//!
//!
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod cli {
    //! A command line interface for `kwhue`.
    //!
    //! Commands
    //! --------
    //! - user add
    //! - light list
    //! - light on [id]
    //! - light off [id]
    //! - light toggle [id]
    pub mod commands {
        pub mod admin {
            //! admin commands
            //!
            //! > admin init
            //! > admin check
            //!
            use crate::hue::bridge::bridge_status;
            use crate::hue::bridge::BridgeStatus;
            use crate::{
                config::{store_app_cfg, AppConfig},
                hue::bridge::Bridge,
            };
            use colored::Colorize;
            /// Reset the app.
            ///
            /// Clearing the config file forces it to reinitialise next time it starts up.
            pub async fn reset() {
                let cfg = AppConfig::default();
                store_app_cfg(&cfg);
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["", ": ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"info".green().bold()),
                            ::core::fmt::ArgumentV1::new_display(
                                &"app reset, restart app to reinitialse".bold(),
                            ),
                        ],
                    ));
                };
            }
            /// Print the status of the Hue bridge to the terminal
            pub async fn info(bridge: &Bridge) {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["", "\n"],
                        &[::core::fmt::ArgumentV1::new_display(&"Status".bold())],
                    ));
                };
                match bridge_status().await {
                    BridgeStatus::CONNECTED => {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n\n"],
                            &[::core::fmt::ArgumentV1::new_display(&"Connected".green())],
                        ));
                    }
                    BridgeStatus::DISCONNECTED => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&"Disconnected".red())],
                            ));
                        };
                        std::process::exit(1);
                    }
                }
                match bridge.config_info().await {
                    Ok(data) => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&"ID".bold())],
                            ));
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n\n"],
                                &[::core::fmt::ArgumentV1::new_display(&data.bridge_id)],
                            ));
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&"Software".bold())],
                            ));
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n\n"],
                                &[::core::fmt::ArgumentV1::new_display(
                                    &data.software_version(),
                                )],
                            ));
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&"IP address".bold())],
                            ));
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["", "\n\n"],
                                &[::core::fmt::ArgumentV1::new_display(&data.ipaddress)],
                            ));
                        };
                    }
                    Err(_) => ::core::panicking::panic("not yet implemented"),
                }
            }
        }
        pub mod light {
            use crate::hue::{bridge::Bridge, lights::LightState};
            use colored::*;
            use itertools::Itertools;
            /// COMMAND: List all lights
            ///
            pub async fn all(bridge: &Bridge) {
                {}
                let __tracing_attr_span = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "all",
                                "kwhue::cli::commands::light",
                                tracing::Level::INFO,
                                Some("src/cli/commands/light.rs"),
                                Some(7u32),
                                Some("kwhue::cli::commands::light"),
                                ::tracing_core::field::FieldSet::new(
                                    &["ip"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            interest = CALLSITE.interest();
                            !interest.is_never()
                        }
                        && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&display(&bridge.ip_address) as &dyn Value),
                            )])
                        })
                    } else {
                        let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                        {};
                        span
                    }
                };
                let __tracing_instrument_future = async move {
                    #[allow(
                        unreachable_code,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value
                    )]
                    if false {
                        let __tracing_attr_fake_return : () = :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["internal error: entered unreachable code: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: core :: fmt :: Arguments :: new_v1 (& ["this is just for type inference, and is unreachable code"] , & []))])) ;
                        return __tracing_attr_fake_return;
                    }
                    {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/cli/commands/light.rs:9",
                                        "kwhue::cli::commands::light",
                                        ::tracing::Level::DEBUG,
                                        Some("src/cli/commands/light.rs"),
                                        Some(9u32),
                                        Some("kwhue::cli::commands::light"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["listing all lights"],
                                            &[],
                                        )
                                            as &dyn Value),
                                    )])
                                });
                            } else {
                            }
                        };
                        let lights = bridge.get_lights().await.unwrap();
                        for light_id in lights.keys().sorted() {
                            let name = lights[light_id].name.clone();
                            let state = match lights[light_id].state.on.unwrap() {
                                true => "ON".bright_yellow(),
                                false => "OFF".bright_blue(),
                            };
                            let reachable = match lights[light_id].state.reachable.unwrap() {
                                true => "OK".green(),
                                false => "UNREACHABLE".red(),
                            };
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                                    &["[", "] ", " - ", " (", ")\n"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&light_id),
                                        ::core::fmt::ArgumentV1::new_display(&name),
                                        ::core::fmt::ArgumentV1::new_display(&state),
                                        ::core::fmt::ArgumentV1::new_display(&reachable),
                                    ],
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Right,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Is(2usize),
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Left,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Is(20usize),
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 2usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Left,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Is(3usize),
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 3usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                    unsafe { ::core::fmt::UnsafeArg::new() },
                                ));
                            };
                        }
                    }
                };
                if !__tracing_attr_span.is_disabled() {
                    tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
                } else {
                    __tracing_instrument_future.await
                }
            }
            /// COMMAND: Turn on light with id
            ///
            pub async fn on(bridge: &Bridge, id: &u32) {
                let current_state = bridge.get_state_for_light(id).await.unwrap();
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                        &["", "\n"],
                        &[::core::fmt::ArgumentV1::new_debug(&current_state)],
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        }],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ));
                };
                let new_state = LightState {
                    on: Some(true),
                    ..current_state
                };
                let endpoint = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["lights/", "/state"],
                        &[::core::fmt::ArgumentV1::new_display(&id)],
                    ));
                    res
                };
                bridge.put(&endpoint, &new_state).await.unwrap();
            }
            /// COMMAND: Turn off light with id
            ///
            pub async fn off(bridge: &Bridge, id: &u32) {
                let current_state = bridge.get_state_for_light(id).await.unwrap();
                let new_state = LightState {
                    on: Some(false),
                    ..current_state
                };
                let endpoint = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["lights/", "/state"],
                        &[::core::fmt::ArgumentV1::new_display(&id)],
                    ));
                    res
                };
                bridge.put(&endpoint, &new_state).await.unwrap();
            }
            /// COMMAND: Toggle light with id
            ///
            pub async fn toggle(bridge: &Bridge, id: &u32) {
                let current_state = bridge.get_state_for_light(id).await.unwrap();
                let new_state = LightState {
                    on: Some(!current_state.on.unwrap()),
                    ..current_state
                };
                let endpoint = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["lights/", "/state"],
                        &[::core::fmt::ArgumentV1::new_display(&id)],
                    ));
                    res
                };
                bridge.put(&endpoint, &new_state).await.unwrap();
            }
        }
    }
    use clap::{Parser, Subcommand};
    # [command (author , version , about , long_about = None)]
    #[command(propagate_version = true)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }
    impl clap::Parser for Cli {}
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::CommandFactory for Cli {
        fn command<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("kwhue");
            <Self as clap::Args>::augment_args(__clap_app)
        }
        fn command_for_update<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("kwhue");
            <Self as clap::Args>::augment_args_for_update(__clap_app)
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for Cli {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = Cli {
                command: {
                    <Commands as clap::FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?
                },
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            {
                #[allow(non_snake_case)]
                let command = &mut self.command;
                <Commands as clap::FromArgMatches>::update_from_arg_matches_mut(
                    command,
                    __clap_arg_matches,
                )?;
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::Args for Cli {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("Cli"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("Cli").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(__clap_app);
                let __clap_app = __clap_app
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_app
                    .version("0.1.0")
                    .long_about(None)
                    .propagate_version(true)
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app =
                    __clap_app.group(clap::ArgGroup::new("Cli").multiple(true).args({
                        let members: [clap::Id; 0usize] = [];
                        members
                    }));
                let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(__clap_app);
                let __clap_app = __clap_app;
                __clap_app
                    .version("0.1.0")
                    .long_about(None)
                    .propagate_version(true)
            }
        }
    }
    pub enum Commands {
        /// Admin commands
        #[command(subcommand)]
        Admin(AdminSubcommands),
        /// Light commands
        #[command(subcommand)]
        Light(LightSubcommands),
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for Commands {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) =
                __clap_arg_matches.remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "admin" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Admin(
                        <AdminSubcommands as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ));
                }
                if __clap_name == "light" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Light(
                        <LightSubcommands as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ));
                }
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The subcommand \'", "\' wasn\'t recognized"],
                            &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                        ));
                        res
                    },
                ))
            } else {
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ))
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::Admin(ref mut __clap_arg) if "admin" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Light(ref mut __clap_arg) if "light" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::Subcommand for Commands {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("admin");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <AdminSubcommands as clap::Subcommand>::augment_subcommands(__clap_subcommand)
                };
                let __clap_subcommand = __clap_subcommand
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_subcommand.about("Admin commands").long_about(None)
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("light");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <LightSubcommands as clap::Subcommand>::augment_subcommands(__clap_subcommand)
                };
                let __clap_subcommand = __clap_subcommand
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_subcommand.about("Light commands").long_about(None)
            });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("admin");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <AdminSubcommands as clap::Subcommand>::augment_subcommands_for_update(
                        __clap_subcommand,
                    )
                };
                let __clap_subcommand = __clap_subcommand
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_subcommand.about("Admin commands").long_about(None)
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("light");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <LightSubcommands as clap::Subcommand>::augment_subcommands_for_update(
                        __clap_subcommand,
                    )
                };
                let __clap_subcommand = __clap_subcommand
                    .subcommand_required(true)
                    .arg_required_else_help(true);
                __clap_subcommand.about("Light commands").long_about(None)
            });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "admin" == __clap_name {
                return true;
            }
            if "light" == __clap_name {
                return true;
            }
            false
        }
    }
    pub enum AdminSubcommands {
        /// Reset the Hue bridge
        Reset {},
        /// Check Hue bridge status
        Check {},
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for AdminSubcommands {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) =
                __clap_arg_matches.remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "reset" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Reset {});
                }
                if __clap_name == "check" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Check {});
                }
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The subcommand \'", "\' wasn\'t recognized"],
                            &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                        ));
                        res
                    },
                ))
            } else {
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ))
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::Reset {} if "reset" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Check {} if "check" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::Subcommand for AdminSubcommands {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("reset");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Reset").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                    );
                    __clap_subcommand
                        .about("Reset the Hue bridge")
                        .long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("check");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Check").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                    );
                    __clap_subcommand
                        .about("Check Hue bridge status")
                        .long_about(None)
                }
            });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("reset");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Reset").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                    );
                    __clap_subcommand
                        .about("Reset the Hue bridge")
                        .long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("check");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Check").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                    );
                    __clap_subcommand
                        .about("Check Hue bridge status")
                        .long_about(None)
                }
            });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "reset" == __clap_name {
                return true;
            }
            if "check" == __clap_name {
                return true;
            }
            false
        }
    }
    pub enum LightSubcommands {
        /// List all lights
        List {},
        /// Toggle light with id
        Toggle {
            /// The id of the light to toggle
            id: u32,
        },
        /// Light on with id
        On {
            /// The id of the light to toggle
            id: u32,
        },
        /// Light off with id
        Off {
            /// The id of the light to toggle
            id: u32,
        },
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for LightSubcommands {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) =
                __clap_arg_matches.remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "list" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::List {});
                }
                if __clap_name == "toggle" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Toggle {
                        id: __clap_arg_matches
                            .remove_one::<u32>("id")
                            .map(|s| s)
                            .ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["The following required argument was not provided: "],
                                        &[::core::fmt::ArgumentV1::new_display(&"id")],
                                    ));
                                    res
                                })
                            })
                            .and_then(|s| ::std::result::Result::Ok::<_, clap::Error>(s))?,
                    });
                }
                if __clap_name == "on" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::On {
                        id: __clap_arg_matches
                            .remove_one::<u32>("id")
                            .map(|s| s)
                            .ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["The following required argument was not provided: "],
                                        &[::core::fmt::ArgumentV1::new_display(&"id")],
                                    ));
                                    res
                                })
                            })
                            .and_then(|s| ::std::result::Result::Ok::<_, clap::Error>(s))?,
                    });
                }
                if __clap_name == "off" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Off {
                        id: __clap_arg_matches
                            .remove_one::<u32>("id")
                            .map(|s| s)
                            .ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["The following required argument was not provided: "],
                                        &[::core::fmt::ArgumentV1::new_display(&"id")],
                                    ));
                                    res
                                })
                            })
                            .and_then(|s| ::std::result::Result::Ok::<_, clap::Error>(s))?,
                    });
                }
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The subcommand \'", "\' wasn\'t recognized"],
                            &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                        ));
                        res
                    },
                ))
            } else {
                ::std::result::Result::Err(clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ))
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::List {} if "list" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Toggle { id } if "toggle" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("id") {
                                * id = __clap_arg_matches . remove_one :: < u32 > ("id") . map (| s | s) . ok_or_else (| | clap :: Error :: raw (clap :: error :: ErrorKind :: MissingRequiredArgument , { let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["The following required argument was not provided: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "id")])) ; res })) . and_then (| s | :: std :: result :: Result :: Ok :: < _ , clap :: Error > (s)) ?
                            }
                        }
                    }
                    Self::On { id } if "on" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("id") {
                                * id = __clap_arg_matches . remove_one :: < u32 > ("id") . map (| s | s) . ok_or_else (| | clap :: Error :: raw (clap :: error :: ErrorKind :: MissingRequiredArgument , { let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["The following required argument was not provided: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "id")])) ; res })) . and_then (| s | :: std :: result :: Result :: Ok :: < _ , clap :: Error > (s)) ?
                            }
                        }
                    }
                    Self::Off { id } if "off" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) =
                            __clap_arg_matches.remove_subcommand().unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("id") {
                                * id = __clap_arg_matches . remove_one :: < u32 > ("id") . map (| s | s) . ok_or_else (| | clap :: Error :: raw (clap :: error :: ErrorKind :: MissingRequiredArgument , { let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["The following required argument was not provided: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "id")])) ; res })) . and_then (| s | :: std :: result :: Result :: Ok :: < _ , clap :: Error > (s)) ?
                            }
                        }
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting
    )]
    #[deny(clippy::correctness)]
    impl clap::Subcommand for LightSubcommands {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("list");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("List").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }));
                    __clap_subcommand.about("List all lights").long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("toggle");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Toggle").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }),
                    );
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(true && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand
                        .about("Toggle light with id")
                        .long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("on");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("On").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }));
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(true && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand.about("Light on with id").long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("off");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("Off").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }));
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(true && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand
                        .about("Light off with id")
                        .long_about(None)
                }
            });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("list");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("List").multiple(true).args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }));
                    __clap_subcommand.about("List all lights").long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("toggle");
                {
                    let __clap_subcommand = __clap_subcommand.group(
                        clap::ArgGroup::new("Toggle").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }),
                    );
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand
                        .about("Toggle light with id")
                        .long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("on");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("On").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }));
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand.about("Light on with id").long_about(None)
                }
            });
            let __clap_app = __clap_app.subcommand({
                let __clap_subcommand = clap::Command::new("off");
                {
                    let __clap_subcommand =
                        __clap_subcommand.group(clap::ArgGroup::new("Off").multiple(true).args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                            members
                        }));
                    let __clap_subcommand = __clap_subcommand.arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("id")
                            .value_name("ID")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap::builder::via_prelude::*;
                                let auto = ::clap::builder::_AutoValueParser::<u32>::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.help("The id of the light to toggle").long_help(None);
                        arg
                    });
                    __clap_subcommand
                        .about("Light off with id")
                        .long_about(None)
                }
            });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "list" == __clap_name {
                return true;
            }
            if "toggle" == __clap_name {
                return true;
            }
            if "on" == __clap_name {
                return true;
            }
            if "off" == __clap_name {
                return true;
            }
            false
        }
    }
}
pub mod config {
    /// Handles app configuration
    ///
    use serde::{Deserialize, Serialize};
    use std::net::IpAddr;
    const CONFIG_NAME: &str = "kwhue";
    pub struct AppConfig {
        pub auth_key: Option<String>,
        pub bridge_ipaddr: Option<IpAddr>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AppConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "AppConfig",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "auth_key",
                    &self.auth_key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "bridge_ipaddr",
                    &self.bridge_ipaddr,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AppConfig {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "auth_key" => _serde::__private::Ok(__Field::__field0),
                            "bridge_ipaddr" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"auth_key" => _serde::__private::Ok(__Field::__field0),
                            b"bridge_ipaddr" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AppConfig>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AppConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct AppConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct AppConfig with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<IpAddr>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct AppConfig with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(AppConfig {
                            auth_key: __field0,
                            bridge_ipaddr: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<IpAddr>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "auth_key",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "bridge_ipaddr",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<IpAddr>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("auth_key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("bridge_ipaddr") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(AppConfig {
                            auth_key: __field0,
                            bridge_ipaddr: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["auth_key", "bridge_ipaddr"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AppConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AppConfig>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for AppConfig {
        #[inline]
        fn default() -> AppConfig {
            AppConfig {
                auth_key: ::core::default::Default::default(),
                bridge_ipaddr: ::core::default::Default::default(),
            }
        }
    }
    /// Get the user configuration data
    pub fn get_app_cfg() -> AppConfig {
        confy::load(CONFIG_NAME, None).unwrap()
    }
    /// Store the user configuration data
    pub fn store_app_cfg(cfg: &AppConfig) {
        confy::store(CONFIG_NAME, None, cfg).unwrap();
    }
    /// Get the user configuration data file path
    pub fn get_cfg_file_path() -> String {
        confy::get_configuration_file_path(CONFIG_NAME, None)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
pub mod error {
    pub enum AppError {
        #[error("network error")]
        NetworkError,
        #[error("Hue bridge not found")]
        HueBridgeNotFoundError,
        #[error("Hue bridge ip address not found")]
        HueBridgeAddressNotFoundError,
        #[error("Hue bridge timeout")]
        HueBridgeTimeout,
        #[error("Hue bridge misconfigured")]
        HueBridgeMisconfigured,
        #[error("Hue bridge authorisation key invalid")]
        HueBridgeAuthKeyInvalid,
        #[error("API not found")]
        APINotFound,
        #[error("other error")]
        Other,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AppError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AppError::NetworkError => ::core::fmt::Formatter::write_str(f, "NetworkError"),
                AppError::HueBridgeNotFoundError => {
                    ::core::fmt::Formatter::write_str(f, "HueBridgeNotFoundError")
                }
                AppError::HueBridgeAddressNotFoundError => {
                    ::core::fmt::Formatter::write_str(f, "HueBridgeAddressNotFoundError")
                }
                AppError::HueBridgeTimeout => {
                    ::core::fmt::Formatter::write_str(f, "HueBridgeTimeout")
                }
                AppError::HueBridgeMisconfigured => {
                    ::core::fmt::Formatter::write_str(f, "HueBridgeMisconfigured")
                }
                AppError::HueBridgeAuthKeyInvalid => {
                    ::core::fmt::Formatter::write_str(f, "HueBridgeAuthKeyInvalid")
                }
                AppError::APINotFound => ::core::fmt::Formatter::write_str(f, "APINotFound"),
                AppError::Other => ::core::fmt::Formatter::write_str(f, "Other"),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for AppError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for AppError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                AppError::NetworkError {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["network error"], &[]))
                }
                AppError::HueBridgeNotFoundError {} => __formatter.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Hue bridge not found"], &[]),
                ),
                AppError::HueBridgeAddressNotFoundError {} => __formatter.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Hue bridge ip address not found"], &[]),
                ),
                AppError::HueBridgeTimeout {} => __formatter
                    .write_fmt(::core::fmt::Arguments::new_v1(&["Hue bridge timeout"], &[])),
                AppError::HueBridgeMisconfigured {} => __formatter.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Hue bridge misconfigured"], &[]),
                ),
                AppError::HueBridgeAuthKeyInvalid {} => __formatter.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Hue bridge authorisation key invalid"], &[]),
                ),
                AppError::APINotFound {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["API not found"], &[]))
                }
                AppError::Other {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["other error"], &[]))
                }
            }
        }
    }
}
pub mod hue {
    pub mod bridge {
        //! A Hue bridge
        use super::lights::LightState;
        use crate::config::*;
        use crate::error::AppError;
        use colored::Colorize;
        use futures_util::{pin_mut, stream::StreamExt};
        use mdns::{Record, RecordKind};
        use reqwest::StatusCode;
        use serde::{de::DeserializeOwned, Deserialize, Serialize};
        use std::io::Write;
        use std::{collections::HashMap, net::IpAddr, time::Duration};
        use tokio::time::timeout;
        use tracing::trace;
        /// Represents a response from the bridge
        ///
        /// json example
        ///
        /// { "error": {} } // Error<T>
        /// { "success": {} } // Success<T>
        #[serde(untagged)]
        enum Response<T, E> {
            Error(Error<E>),
            Success(Success<T>),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, T, E> _serde::Deserialize<'de> for Response<T, E>
            where
                T: _serde::Deserialize<'de>,
                E: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content =
                        match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                        <Error<E> as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        Response::Error,
                    ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                        <Success<T> as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                                &__content,
                            ),
                        ),
                        Response::Success,
                    ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(_serde::de::Error::custom(
                        "data did not match any variant of untagged enum Response",
                    ))
                }
            }
        };
        #[automatically_derived]
        impl<T: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Response<T, E> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Response::Error(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Error", &__self_0)
                    }
                    Response::Success(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Success", &__self_0)
                    }
                }
            }
        }
        /// Bridge 'error' response
        struct Error<T> {
            error: T,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, T> _serde::Deserialize<'de> for Error<T>
            where
                T: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "error" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"error" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de, T>
                    where
                        T: _serde::Deserialize<'de>,
                    {
                        marker: _serde::__private::PhantomData<Error<T>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                    where
                        T: _serde::Deserialize<'de>,
                    {
                        type Value = Error<T>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Error")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<T>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Error with 1 element",
                                            ),
                                        );
                                    }
                                };
                            _serde::__private::Ok(Error { error: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<T> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "error",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<T>(&mut __map)
                                            {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("error") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Error { error: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["error"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Error",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Error<T>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Error<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Error",
                    "error",
                    &&self.error,
                )
            }
        }
        /// Bridge 'sucess' response
        struct Success<T> {
            success: T,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, T> _serde::Deserialize<'de> for Success<T>
            where
                T: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "success" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"success" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de, T>
                    where
                        T: _serde::Deserialize<'de>,
                    {
                        marker: _serde::__private::PhantomData<Success<T>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                    where
                        T: _serde::Deserialize<'de>,
                    {
                        type Value = Success<T>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Success")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<T>(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Success with 1 element",
                                            ),
                                        );
                                    }
                                };
                            _serde::__private::Ok(Success { success: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<T> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "success",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<T>(&mut __map)
                                            {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("success") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Success { success: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["success"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Success",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Success<T>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Success<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Success",
                    "success",
                    &&self.success,
                )
            }
        }
        /// Bridge authorisation key data
        pub struct AuthKeyResponse {
            pub username: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for AuthKeyResponse {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "AuthKeyResponse",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "username",
                        &self.username,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for AuthKeyResponse {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "username" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"username" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<AuthKeyResponse>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AuthKeyResponse;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct AuthKeyResponse",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AuthKeyResponse with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(AuthKeyResponse { username: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("username") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(AuthKeyResponse { username: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["username"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AuthKeyResponse",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<AuthKeyResponse>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for AuthKeyResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AuthKeyResponse",
                    "username",
                    &&self.username,
                )
            }
        }
        /// Bridge error data
        struct BasicError {
            #[serde(rename = "type")]
            error_type: u32,
            description: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for BasicError {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "type" => _serde::__private::Ok(__Field::__field0),
                                "description" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"type" => _serde::__private::Ok(__Field::__field0),
                                b"description" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<BasicError>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = BasicError;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct BasicError",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct BasicError with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct BasicError with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(BasicError {
                                error_type: __field0,
                                description: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("type") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("description") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(BasicError {
                                error_type: __field0,
                                description: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["type", "description"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "BasicError",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<BasicError>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for BasicError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "BasicError",
                    "error_type",
                    &&self.error_type,
                    "description",
                    &&self.description,
                )
            }
        }
        pub enum BridgeStatus {
            CONNECTED,
            DISCONNECTED,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BridgeStatus {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    BridgeStatus::CONNECTED => ::core::fmt::Formatter::write_str(f, "CONNECTED"),
                    BridgeStatus::DISCONNECTED => {
                        ::core::fmt::Formatter::write_str(f, "DISCONNECTED")
                    }
                }
            }
        }
        /// A Hue Bridge client providing API commands
        ///
        pub struct Bridge {
            pub ip_address: IpAddr,
            pub auth_key: String,
            pub client: reqwest::Client,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Bridge {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Bridge",
                    "ip_address",
                    &&self.ip_address,
                    "auth_key",
                    &&self.auth_key,
                    "client",
                    &&self.client,
                )
            }
        }
        pub struct ConfigInfo {
            #[serde(rename = "bridgeid")]
            pub bridge_id: String,
            pub apiversion: String,
            pub swversion: String,
            pub ipaddress: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ConfigInfo {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "bridgeid" => _serde::__private::Ok(__Field::__field0),
                                "apiversion" => _serde::__private::Ok(__Field::__field1),
                                "swversion" => _serde::__private::Ok(__Field::__field2),
                                "ipaddress" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"bridgeid" => _serde::__private::Ok(__Field::__field0),
                                b"apiversion" => _serde::__private::Ok(__Field::__field1),
                                b"swversion" => _serde::__private::Ok(__Field::__field2),
                                b"ipaddress" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ConfigInfo>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ConfigInfo;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ConfigInfo",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ConfigInfo with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ConfigInfo with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ConfigInfo with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ConfigInfo with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ConfigInfo {
                                bridge_id: __field0,
                                apiversion: __field1,
                                swversion: __field2,
                                ipaddress: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "bridgeid",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "apiversion",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "swversion",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "ipaddress",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("bridgeid") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("apiversion") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("swversion") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("ipaddress") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ConfigInfo {
                                bridge_id: __field0,
                                apiversion: __field1,
                                swversion: __field2,
                                ipaddress: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["bridgeid", "apiversion", "swversion", "ipaddress"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ConfigInfo",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ConfigInfo>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfigInfo {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ConfigInfo",
                    "bridge_id",
                    &&self.bridge_id,
                    "apiversion",
                    &&self.apiversion,
                    "swversion",
                    &&self.swversion,
                    "ipaddress",
                    &&self.ipaddress,
                )
            }
        }
        impl ConfigInfo {
            pub fn software_version(&self) -> String {
                let parts: Vec<&str> = self.apiversion.split(".").collect();
                {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["", ".", "."],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&parts[0]),
                            ::core::fmt::ArgumentV1::new_display(&parts[1]),
                            ::core::fmt::ArgumentV1::new_display(&self.swversion),
                        ],
                    ));
                    res
                }
            }
        }
        impl Bridge {
            /// Create a new Hue bridge instance
            pub async fn new() -> Self {
                if !is_configured() {
                    match configure().await {
                        Ok(_) => {}
                        Err(err) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["", ": ", "\n\n"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&"error".red().bold()),
                                        ::core::fmt::ArgumentV1::new_display(
                                            &err.to_string().bold(),
                                        ),
                                    ],
                                ));
                            };
                            std::process::exit(1);
                        }
                    }
                }
                let cfg = get_app_cfg();
                let ip_address = cfg.bridge_ipaddr.unwrap();
                let auth_key = cfg.auth_key.unwrap();
                let client = reqwest::Client::builder().build().unwrap();
                Self {
                    ip_address,
                    auth_key,
                    client,
                }
            }
        }
        impl Bridge {
            /// Gets an endpoint response and deserialises it.
            pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, AppError> {
                {}
                let __tracing_attr_span = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "get",
                                "kwhue::hue::bridge",
                                tracing::Level::INFO,
                                Some("src/hue/bridge.rs"),
                                Some(116u32),
                                Some("kwhue::hue::bridge"),
                                ::tracing_core::field::FieldSet::new(
                                    &["endpoint"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            interest = CALLSITE.interest();
                            !interest.is_never()
                        }
                        && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&endpoint as &dyn Value),
                            )])
                        })
                    } else {
                        let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                        {};
                        span
                    }
                };
                let __tracing_instrument_future = async move {
                    #[allow(
                        unreachable_code,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value
                    )]
                    if false {
                        let __tracing_attr_fake_return : Result < T , AppError > = :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["internal error: entered unreachable code: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: core :: fmt :: Arguments :: new_v1 (& ["this is just for type inference, and is unreachable code"] , & []))])) ;
                        return __tracing_attr_fake_return;
                    }
                    {
                        let cfg = get_app_cfg();
                        let url = {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["http://", "/api/", "/"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&self.ip_address),
                                    ::core::fmt::ArgumentV1::new_display(&cfg.auth_key.unwrap()),
                                    ::core::fmt::ArgumentV1::new_display(&endpoint),
                                ],
                            ));
                            res
                        };
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/hue/bridge.rs:126",
                                        "kwhue::hue::bridge",
                                        ::tracing::Level::TRACE,
                                        Some("src/hue/bridge.rs"),
                                        Some(126u32),
                                        Some("kwhue::hue::bridge"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "url"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &::core::fmt::Arguments::new_v1(&["fetching"], &[])
                                                    as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&url as &dyn Value),
                                        ),
                                    ])
                                });
                            } else {
                            }
                        };
                        let resp = self
                            .client
                            .get(url)
                            .timeout(std::time::Duration::from_millis(500))
                            .send()
                            .await
                            .map_err(|_| AppError::NetworkError)
                            .unwrap();
                        let status = resp.status();
                        match status {
                            StatusCode::OK => Ok(resp.json().await.map_err(|_| AppError::Other)?),
                            StatusCode::NOT_FOUND => Err(AppError::APINotFound),
                            _ => Err(AppError::Other),
                        }
                    }
                };
                if !__tracing_attr_span.is_disabled() {
                    tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
                } else {
                    __tracing_instrument_future.await
                }
            }
        }
        impl Bridge {
            /// Puts the given data to the given endpoint
            pub async fn put(&self, endpoint: &str, data: &LightState) -> Result<(), AppError> {
                let cfg = get_app_cfg();
                let url = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["http://", "/api/", "/"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&self.ip_address),
                            ::core::fmt::ArgumentV1::new_display(&cfg.auth_key.unwrap()),
                            ::core::fmt::ArgumentV1::new_display(&endpoint),
                        ],
                    ));
                    res
                };
                let body_json = serde_json::to_string(data).unwrap();
                let resp = self.client.put(&url).body(body_json).send().await.unwrap();
                match resp.status() {
                    StatusCode::OK => Ok(()),
                    StatusCode::NOT_FOUND => Err(AppError::APINotFound),
                    _ => Err(AppError::Other),
                }
            }
            pub async fn config_info(&self) -> Result<ConfigInfo, AppError> {
                let data: ConfigInfo = self.get("config").await?;
                Ok(data)
            }
        }
        /// Return true if config file contains an ip address and auth key.
        fn is_configured() -> bool {
            let cfg = get_app_cfg();
            cfg.bridge_ipaddr.is_some() && cfg.auth_key.is_some()
        }
        /// Gets the IP address, creates an auth_key, and saves both to the config file.
        async fn configure() -> Result<(), AppError> {
            let mut cfg = get_app_cfg();
            let ipaddr = match get_bridge_ipaddr().await {
                Ok(ipaddr) => ipaddr,
                Err(err) => return Err(err),
            };
            let auth_key = match create_new_auth_key(ipaddr).await {
                Ok(auth_key) => auth_key,
                Err(err) => return Err(err),
            };
            cfg.bridge_ipaddr = Some(ipaddr);
            cfg.auth_key = Some(auth_key);
            store_app_cfg(&cfg);
            Ok(())
        }
        /// Gets the Hue Bridge ip address.
        pub async fn get_bridge_ipaddr() -> Result<IpAddr, AppError> {
            const SERVICE_NAME: &str = "_hue._tcp.local";
            let responses = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))
                .map_err(|_| AppError::Other)?
                .listen();
            let mut responses = responses;
            #[allow(unused_mut)]
            let mut responses =
                unsafe { ::pin_utils::core_reexport::pin::Pin::new_unchecked(&mut responses) };
            let response = match timeout(Duration::from_secs(5), responses.next()).await {
                Ok(r) => Ok(r),
                Err(_) => Err(AppError::HueBridgeTimeout),
            };
            match response {
                Ok(r) => match r {
                    Some(Ok(response)) => {
                        let addr = response.records().filter_map(self::to_ip_addr).next();
                        if let Some(addr) = addr {
                            Ok(addr)
                        } else {
                            Err(AppError::HueBridgeAddressNotFoundError)
                        }
                    }
                    Some(Err(_)) => ::core::panicking::panic("not yet implemented"),
                    None => Err(AppError::Other),
                },
                Err(err) => Err(err),
            }
        }
        /// Creates a hub authorisation key.
        ///
        /// See [Hue Configuration API](https://developers.meethue.com/develop/hue-api/7-configuration-api/)
        pub async fn create_new_auth_key(ip_addr: IpAddr) -> Result<String, AppError> {
            let url = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["http://", "/api"],
                    &[::core::fmt::ArgumentV1::new_display(&ip_addr)],
                ));
                res
            };
            let client = reqwest::Client::new();
            let mut params = HashMap::new();
            params.insert("devicetype", "kwhue_app rust_app");
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["Press link button"], &[]));
            };
            std::io::stdout().flush().unwrap();
            let response = loop {
                let resp = client.post(&url).json(&params).send().await.unwrap();
                let status = resp.status();
                let mut data: Vec<Response<AuthKeyResponse, BasicError>> = match status {
                    StatusCode::OK => resp.json().await.map_err(|err| {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["error: ", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&err)],
                            ));
                        };
                        AppError::HueBridgeAuthKeyInvalid
                    }),
                    StatusCode::NOT_FOUND => Err(AppError::APINotFound),
                    _ => Err(AppError::Other),
                }
                .unwrap();
                match data.pop().unwrap() {
                    Response::Error(e) => {
                        if e.error.error_type != 101 {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Error: ", "\n"],
                                    &[::core::fmt::ArgumentV1::new_display(&e.error.description)],
                                ));
                            };
                        }
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["."], &[]));
                        };
                        std::io::stdout().flush().unwrap();
                    }
                    Response::Success(s) => break s.success,
                }
            };
            Ok(response.username)
        }
        ///
        pub async fn bridge_status() -> BridgeStatus {
            let result = get_bridge_ipaddr().await;
            match result {
                Ok(_) => BridgeStatus::CONNECTED,
                Err(_) => BridgeStatus::DISCONNECTED,
            }
        }
        fn to_ip_addr(record: &Record) -> Option<IpAddr> {
            match record.kind {
                RecordKind::A(addr) => Some(addr.into()),
                RecordKind::AAAA(addr) => Some(addr.into()),
                _ => None,
            }
        }
    }
    pub mod lights {
        ///
        /// Implement Lights API
        ///
        /// see: https://developers.meethue.com/develop/hue-api/lights-api/#get-new-lights
        ///
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use crate::error::AppError;
        use crate::hue::bridge::Bridge;
        type JsonMap = HashMap<u32, Light>;
        impl Bridge {
            pub async fn get_lights(&self) -> Result<JsonMap, AppError> {
                {}
                let __tracing_attr_span = {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "get_lights",
                                "kwhue::hue::lights",
                                tracing::Level::INFO,
                                Some("src/hue/lights.rs"),
                                Some(15u32),
                                Some("kwhue::hue::lights"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            interest = CALLSITE.interest();
                            !interest.is_never()
                        }
                        && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                    {
                        let meta = CALLSITE.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                        {};
                        span
                    }
                };
                let __tracing_instrument_future = async move {
                    #[allow(
                        unreachable_code,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value
                    )]
                    if false {
                        let __tracing_attr_fake_return : Result < JsonMap , AppError > = :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["internal error: entered unreachable code: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& :: core :: fmt :: Arguments :: new_v1 (& ["this is just for type inference, and is unreachable code"] , & []))])) ;
                        return __tracing_attr_fake_return;
                    }
                    {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/hue/lights.rs:17",
                                        "kwhue::hue::lights",
                                        ::tracing::Level::DEBUG,
                                        Some("src/hue/lights.rs"),
                                        Some(17u32),
                                        Some("kwhue::hue::lights"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["getting lights"],
                                            &[],
                                        )
                                            as &dyn Value),
                                    )])
                                });
                            } else {
                            }
                        };
                        let data: JsonMap = self.get("lights").await?;
                        Ok(data)
                    }
                };
                if !__tracing_attr_span.is_disabled() {
                    tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
                } else {
                    __tracing_instrument_future.await
                }
            }
        }
        impl Bridge {
            pub async fn get_state_for_light(&self, id: &u32) -> Result<LightState, AppError> {
                let url = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["lights/"],
                        &[::core::fmt::ArgumentV1::new_display(&id)],
                    ));
                    res
                };
                let state_response: StateResponse = self.get(&url).await?;
                Ok(state_response.state)
            }
        }
        /// Hue light schema
        ///
        pub struct Light {
            capabilities: Capabilities,
            config: Config,
            pub state: LightState,
            pub name: String,
            #[serde(rename = "manufacturername")]
            manufacturer_name: String,
            #[serde(rename = "modelid")]
            model_id: String,
            #[serde(rename = "productid")]
            product_id: Option<String>,
            #[serde(rename = "productname")]
            product_name: String,
            #[serde(rename = "type")]
            light_type: String,
            #[serde(rename = "uniqueid")]
            unique_id: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Light {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "capabilities" => _serde::__private::Ok(__Field::__field0),
                                "config" => _serde::__private::Ok(__Field::__field1),
                                "state" => _serde::__private::Ok(__Field::__field2),
                                "name" => _serde::__private::Ok(__Field::__field3),
                                "manufacturername" => _serde::__private::Ok(__Field::__field4),
                                "modelid" => _serde::__private::Ok(__Field::__field5),
                                "productid" => _serde::__private::Ok(__Field::__field6),
                                "productname" => _serde::__private::Ok(__Field::__field7),
                                "type" => _serde::__private::Ok(__Field::__field8),
                                "uniqueid" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"capabilities" => _serde::__private::Ok(__Field::__field0),
                                b"config" => _serde::__private::Ok(__Field::__field1),
                                b"state" => _serde::__private::Ok(__Field::__field2),
                                b"name" => _serde::__private::Ok(__Field::__field3),
                                b"manufacturername" => _serde::__private::Ok(__Field::__field4),
                                b"modelid" => _serde::__private::Ok(__Field::__field5),
                                b"productid" => _serde::__private::Ok(__Field::__field6),
                                b"productname" => _serde::__private::Ok(__Field::__field7),
                                b"type" => _serde::__private::Ok(__Field::__field8),
                                b"uniqueid" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Light>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Light;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Light")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Capabilities,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<Config>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                LightState,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Light with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Light {
                                capabilities: __field0,
                                config: __field1,
                                state: __field2,
                                name: __field3,
                                manufacturer_name: __field4,
                                model_id: __field5,
                                product_id: __field6,
                                product_name: __field7,
                                light_type: __field8,
                                unique_id: __field9,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Capabilities> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Config> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<LightState> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field6: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "capabilities",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Capabilities>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "config",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Config>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "state",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<LightState>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "manufacturername",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "modelid",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "productid",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "productname",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "type",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "uniqueid",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("capabilities") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("config") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("state") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("manufacturername") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("modelid") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("productid") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("productname") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("type") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("uniqueid") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Light {
                                capabilities: __field0,
                                config: __field1,
                                state: __field2,
                                name: __field3,
                                manufacturer_name: __field4,
                                model_id: __field5,
                                product_id: __field6,
                                product_name: __field7,
                                light_type: __field8,
                                unique_id: __field9,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "capabilities",
                        "config",
                        "state",
                        "name",
                        "manufacturername",
                        "modelid",
                        "productid",
                        "productname",
                        "type",
                        "uniqueid",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Light",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Light>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Light {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "capabilities",
                    "config",
                    "state",
                    "name",
                    "manufacturer_name",
                    "model_id",
                    "product_id",
                    "product_name",
                    "light_type",
                    "unique_id",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.capabilities,
                    &&self.config,
                    &&self.state,
                    &&self.name,
                    &&self.manufacturer_name,
                    &&self.model_id,
                    &&self.product_id,
                    &&self.product_name,
                    &&self.light_type,
                    &&self.unique_id,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Light", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Light {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Light {
            #[inline]
            fn eq(&self, other: &Light) -> bool {
                self.capabilities == other.capabilities
                    && self.config == other.config
                    && self.state == other.state
                    && self.name == other.name
                    && self.manufacturer_name == other.manufacturer_name
                    && self.model_id == other.model_id
                    && self.product_id == other.product_id
                    && self.product_name == other.product_name
                    && self.light_type == other.light_type
                    && self.unique_id == other.unique_id
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Light {
            #[inline]
            fn clone(&self) -> Light {
                Light {
                    capabilities: ::core::clone::Clone::clone(&self.capabilities),
                    config: ::core::clone::Clone::clone(&self.config),
                    state: ::core::clone::Clone::clone(&self.state),
                    name: ::core::clone::Clone::clone(&self.name),
                    manufacturer_name: ::core::clone::Clone::clone(&self.manufacturer_name),
                    model_id: ::core::clone::Clone::clone(&self.model_id),
                    product_id: ::core::clone::Clone::clone(&self.product_id),
                    product_name: ::core::clone::Clone::clone(&self.product_name),
                    light_type: ::core::clone::Clone::clone(&self.light_type),
                    unique_id: ::core::clone::Clone::clone(&self.unique_id),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Light {
            #[inline]
            fn partial_cmp(&self, other: &Light) -> ::core::option::Option<::core::cmp::Ordering> {
                match :: core :: cmp :: PartialOrd :: partial_cmp (& self . capabilities , & other . capabilities) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . config , & other . config) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . state , & other . state) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . name , & other . name) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . manufacturer_name , & other . manufacturer_name) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . model_id , & other . model_id) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . product_id , & other . product_id) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . product_name , & other . product_name) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . light_type , & other . light_type) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => :: core :: cmp :: PartialOrd :: partial_cmp (& self . unique_id , & other . unique_id) , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , }
            }
        }
        pub struct Capabilities {
            certified: bool,
            control: Control,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Capabilities {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "certified" => _serde::__private::Ok(__Field::__field0),
                                "control" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"certified" => _serde::__private::Ok(__Field::__field0),
                                b"control" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Capabilities>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Capabilities;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Capabilities",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Capabilities with 2 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<Control>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Capabilities with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Capabilities {
                                certified: __field0,
                                control: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Control> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "certified",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<bool>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "control",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Control>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("certified") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("control") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Capabilities {
                                certified: __field0,
                                control: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["certified", "control"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Capabilities",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Capabilities>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Capabilities {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Capabilities",
                    "certified",
                    &&self.certified,
                    "control",
                    &&self.control,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Capabilities {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Capabilities {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<bool>;
                let _: ::core::cmp::AssertParamIsEq<Control>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Capabilities {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Capabilities {
            #[inline]
            fn eq(&self, other: &Capabilities) -> bool {
                self.certified == other.certified && self.control == other.control
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Capabilities {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.certified, state);
                ::core::hash::Hash::hash(&self.control, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Capabilities {
            #[inline]
            fn clone(&self) -> Capabilities {
                Capabilities {
                    certified: ::core::clone::Clone::clone(&self.certified),
                    control: ::core::clone::Clone::clone(&self.control),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Capabilities {
            #[inline]
            fn cmp(&self, other: &Capabilities) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.certified, &other.certified) {
                    ::core::cmp::Ordering::Equal => {
                        ::core::cmp::Ord::cmp(&self.control, &other.control)
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Capabilities {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Capabilities,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.certified, &other.certified) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.control, &other.control)
                    }
                    cmp => cmp,
                }
            }
        }
        pub struct Control {
            #[serde(rename = "ct")]
            colour_temperature: Option<Range>,
            #[serde(rename = "maxlumen")]
            max_lumen: u32,
            #[serde(rename = "mindimlevel")]
            min_im_level: u32,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Control {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "ct" => _serde::__private::Ok(__Field::__field0),
                                "maxlumen" => _serde::__private::Ok(__Field::__field1),
                                "mindimlevel" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"ct" => _serde::__private::Ok(__Field::__field0),
                                b"maxlumen" => _serde::__private::Ok(__Field::__field1),
                                b"mindimlevel" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Control>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Control;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Control")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<Range>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Control with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Control with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Control with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Control {
                                colour_temperature: __field0,
                                max_lumen: __field1,
                                min_im_level: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<Range>> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "ct",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<Range>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "maxlumen",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "mindimlevel",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("ct") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("maxlumen") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mindimlevel") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Control {
                                colour_temperature: __field0,
                                max_lumen: __field1,
                                min_im_level: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["ct", "maxlumen", "mindimlevel"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Control",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Control>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Control {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Control",
                    "colour_temperature",
                    &&self.colour_temperature,
                    "max_lumen",
                    &&self.max_lumen,
                    "min_im_level",
                    &&self.min_im_level,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Control {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Control {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Option<Range>>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Control {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Control {
            #[inline]
            fn eq(&self, other: &Control) -> bool {
                self.colour_temperature == other.colour_temperature
                    && self.max_lumen == other.max_lumen
                    && self.min_im_level == other.min_im_level
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Control {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.colour_temperature, state);
                ::core::hash::Hash::hash(&self.max_lumen, state);
                ::core::hash::Hash::hash(&self.min_im_level, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Control {
            #[inline]
            fn clone(&self) -> Control {
                Control {
                    colour_temperature: ::core::clone::Clone::clone(&self.colour_temperature),
                    max_lumen: ::core::clone::Clone::clone(&self.max_lumen),
                    min_im_level: ::core::clone::Clone::clone(&self.min_im_level),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Control {
            #[inline]
            fn cmp(&self, other: &Control) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.colour_temperature, &other.colour_temperature) {
                    ::core::cmp::Ordering::Equal => {
                        match ::core::cmp::Ord::cmp(&self.max_lumen, &other.max_lumen) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(&self.min_im_level, &other.min_im_level)
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Control {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Control,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(
                    &self.colour_temperature,
                    &other.colour_temperature,
                ) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(
                            &self.max_lumen,
                            &other.max_lumen,
                        ) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &self.min_im_level,
                                    &other.min_im_level,
                                )
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        pub struct Range {
            max: u32,
            min: u32,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Range {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "max" => _serde::__private::Ok(__Field::__field0),
                                "min" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"max" => _serde::__private::Ok(__Field::__field0),
                                b"min" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Range>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Range;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Range")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Range with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Range with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Range {
                                max: __field0,
                                min: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "max",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "min",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("max") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("min") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Range {
                                max: __field0,
                                min: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["max", "min"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Range",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Range>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Range {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "Range", "max", &&self.max, "min", &&self.min,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Range {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Range {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Range {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Range {
            #[inline]
            fn eq(&self, other: &Range) -> bool {
                self.max == other.max && self.min == other.min
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Range {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.max, state);
                ::core::hash::Hash::hash(&self.min, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Range {
            #[inline]
            fn clone(&self) -> Range {
                Range {
                    max: ::core::clone::Clone::clone(&self.max),
                    min: ::core::clone::Clone::clone(&self.min),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Range {
            #[inline]
            fn cmp(&self, other: &Range) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.max, &other.max) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.min, &other.min),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Range {
            #[inline]
            fn partial_cmp(&self, other: &Range) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.max, &other.max) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.min, &other.min)
                    }
                    cmp => cmp,
                }
            }
        }
        pub struct Config {
            archetype: String,
            direction: String,
            function: String,
            startup: Startup,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Config {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "archetype" => _serde::__private::Ok(__Field::__field0),
                                "direction" => _serde::__private::Ok(__Field::__field1),
                                "function" => _serde::__private::Ok(__Field::__field2),
                                "startup" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"archetype" => _serde::__private::Ok(__Field::__field0),
                                b"direction" => _serde::__private::Ok(__Field::__field1),
                                b"function" => _serde::__private::Ok(__Field::__field2),
                                b"startup" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Config>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Config;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Config")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Config with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Config with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Config with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<Startup>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Config with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Config {
                                archetype: __field0,
                                direction: __field1,
                                function: __field2,
                                startup: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Startup> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "archetype",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "direction",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "function",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "startup",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Startup>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("archetype") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("direction") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("function") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("startup") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Config {
                                archetype: __field0,
                                direction: __field1,
                                function: __field2,
                                startup: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["archetype", "direction", "function", "startup"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Config",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Config>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Config {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Config",
                    "archetype",
                    &&self.archetype,
                    "direction",
                    &&self.direction,
                    "function",
                    &&self.function,
                    "startup",
                    &&self.startup,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Config {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Config {
            #[inline]
            fn eq(&self, other: &Config) -> bool {
                self.archetype == other.archetype
                    && self.direction == other.direction
                    && self.function == other.function
                    && self.startup == other.startup
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Config {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.archetype, state);
                ::core::hash::Hash::hash(&self.direction, state);
                ::core::hash::Hash::hash(&self.function, state);
                ::core::hash::Hash::hash(&self.startup, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Config {
            #[inline]
            fn clone(&self) -> Config {
                Config {
                    archetype: ::core::clone::Clone::clone(&self.archetype),
                    direction: ::core::clone::Clone::clone(&self.direction),
                    function: ::core::clone::Clone::clone(&self.function),
                    startup: ::core::clone::Clone::clone(&self.startup),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Config {
            #[inline]
            fn partial_cmp(&self, other: &Config) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.archetype, &other.archetype) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        match ::core::cmp::PartialOrd::partial_cmp(
                            &self.direction,
                            &other.direction,
                        ) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                match ::core::cmp::PartialOrd::partial_cmp(
                                    &self.function,
                                    &other.function,
                                ) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &self.startup,
                                            &other.startup,
                                        )
                                    }
                                    cmp => cmp,
                                }
                            }
                            cmp => cmp,
                        }
                    }
                    cmp => cmp,
                }
            }
        }
        struct Startup {
            configured: bool,
            mode: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Startup {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "configured" => _serde::__private::Ok(__Field::__field0),
                                "mode" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"configured" => _serde::__private::Ok(__Field::__field0),
                                b"mode" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Startup>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Startup;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Startup")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Startup with 2 elements",
                                            ),
                                        );
                                    }
                                };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<String>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Startup with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Startup {
                                configured: __field0,
                                mode: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "configured",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<bool>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "mode",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<String>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("configured") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mode") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Startup {
                                configured: __field0,
                                mode: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["configured", "mode"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Startup",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Startup>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Startup {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Startup",
                    "configured",
                    &&self.configured,
                    "mode",
                    &&self.mode,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Startup {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Startup {
            #[inline]
            fn eq(&self, other: &Startup) -> bool {
                self.configured == other.configured && self.mode == other.mode
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Startup {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.configured, state);
                ::core::hash::Hash::hash(&self.mode, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Startup {
            #[inline]
            fn clone(&self) -> Startup {
                Startup {
                    configured: ::core::clone::Clone::clone(&self.configured),
                    mode: ::core::clone::Clone::clone(&self.mode),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Startup {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Startup,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.configured, &other.configured) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.mode, &other.mode)
                    }
                    cmp => cmp,
                }
            }
        }
        #[builder(setter(strip_option))]
        #[builder(build_fn(validate = "Self::validate"))]
        pub struct LightState {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub on: Option<bool>,
            #[serde(rename = "bri")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub brightness: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hue: Option<u32>,
            #[serde(rename = "sat")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub saturation: Option<u8>,
            #[serde(rename = "colormode")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub colour_mode: Option<String>,
            #[serde(rename = "ct")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub colour_temperature: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub xy: Option<XY>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mode: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub reachable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub alert: Option<String>,
        }
        #[automatically_derived]
        impl ::core::default::Default for LightState {
            #[inline]
            fn default() -> LightState {
                LightState {
                    on: ::core::default::Default::default(),
                    brightness: ::core::default::Default::default(),
                    hue: ::core::default::Default::default(),
                    saturation: ::core::default::Default::default(),
                    colour_mode: ::core::default::Default::default(),
                    colour_temperature: ::core::default::Default::default(),
                    xy: ::core::default::Default::default(),
                    mode: ::core::default::Default::default(),
                    reachable: ::core::default::Default::default(),
                    alert: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for LightState {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "LightState",
                        false as usize
                            + if Option::is_none(&self.on) { 0 } else { 1 }
                            + if Option::is_none(&self.brightness) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.hue) { 0 } else { 1 }
                            + if Option::is_none(&self.saturation) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.colour_mode) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.colour_temperature) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.xy) { 0 } else { 1 }
                            + if Option::is_none(&self.mode) { 0 } else { 1 }
                            + if Option::is_none(&self.reachable) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.alert) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.on) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "on",
                            &self.on,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "on") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.brightness) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "bri",
                            &self.brightness,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "bri") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.hue) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "hue",
                            &self.hue,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "hue") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.saturation) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sat",
                            &self.saturation,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "sat") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.colour_mode) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "colormode",
                            &self.colour_mode,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "colormode",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.colour_temperature) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ct",
                            &self.colour_temperature,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "ct") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.xy) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "xy",
                            &self.xy,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "xy") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.mode) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mode",
                            &self.mode,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "mode") {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.reachable) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "reachable",
                            &self.reachable,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "reachable",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.alert) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "alert",
                            &self.alert,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "alert")
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for LightState {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "on" => _serde::__private::Ok(__Field::__field0),
                                "bri" => _serde::__private::Ok(__Field::__field1),
                                "hue" => _serde::__private::Ok(__Field::__field2),
                                "sat" => _serde::__private::Ok(__Field::__field3),
                                "colormode" => _serde::__private::Ok(__Field::__field4),
                                "ct" => _serde::__private::Ok(__Field::__field5),
                                "xy" => _serde::__private::Ok(__Field::__field6),
                                "mode" => _serde::__private::Ok(__Field::__field7),
                                "reachable" => _serde::__private::Ok(__Field::__field8),
                                "alert" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"on" => _serde::__private::Ok(__Field::__field0),
                                b"bri" => _serde::__private::Ok(__Field::__field1),
                                b"hue" => _serde::__private::Ok(__Field::__field2),
                                b"sat" => _serde::__private::Ok(__Field::__field3),
                                b"colormode" => _serde::__private::Ok(__Field::__field4),
                                b"ct" => _serde::__private::Ok(__Field::__field5),
                                b"xy" => _serde::__private::Ok(__Field::__field6),
                                b"mode" => _serde::__private::Ok(__Field::__field7),
                                b"reachable" => _serde::__private::Ok(__Field::__field8),
                                b"alert" => _serde::__private::Ok(__Field::__field9),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<LightState>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = LightState;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct LightState",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<u8>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<XY>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct LightState with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(LightState {
                                on: __field0,
                                brightness: __field1,
                                hue: __field2,
                                saturation: __field3,
                                colour_mode: __field4,
                                colour_temperature: __field5,
                                xy: __field6,
                                mode: __field7,
                                reachable: __field8,
                                alert: __field9,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<bool>> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<u32>> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Option<u32>> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Option<u8>> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Option<u32>> =
                                _serde::__private::None;
                            let mut __field6: _serde::__private::Option<Option<XY>> =
                                _serde::__private::None;
                            let mut __field7: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<bool>> =
                                _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "on",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<bool>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "bri",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<u32>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "hue",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<u32>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sat",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<u8>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "colormode",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "ct",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<u32>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "xy",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<XY>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "mode",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "reachable",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<bool>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "alert",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("on") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("bri") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("hue") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("sat") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("colormode") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("ct") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("xy") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mode") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("reachable") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("alert") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(LightState {
                                on: __field0,
                                brightness: __field1,
                                hue: __field2,
                                saturation: __field3,
                                colour_mode: __field4,
                                colour_temperature: __field5,
                                xy: __field6,
                                mode: __field7,
                                reachable: __field8,
                                alert: __field9,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "on",
                        "bri",
                        "hue",
                        "sat",
                        "colormode",
                        "ct",
                        "xy",
                        "mode",
                        "reachable",
                        "alert",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "LightState",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<LightState>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for LightState {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "on",
                    "brightness",
                    "hue",
                    "saturation",
                    "colour_mode",
                    "colour_temperature",
                    "xy",
                    "mode",
                    "reachable",
                    "alert",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.on,
                    &&self.brightness,
                    &&self.hue,
                    &&self.saturation,
                    &&self.colour_mode,
                    &&self.colour_temperature,
                    &&self.xy,
                    &&self.mode,
                    &&self.reachable,
                    &&self.alert,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "LightState", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LightState {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LightState {
            #[inline]
            fn eq(&self, other: &LightState) -> bool {
                self.on == other.on
                    && self.brightness == other.brightness
                    && self.hue == other.hue
                    && self.saturation == other.saturation
                    && self.colour_mode == other.colour_mode
                    && self.colour_temperature == other.colour_temperature
                    && self.xy == other.xy
                    && self.mode == other.mode
                    && self.reachable == other.reachable
                    && self.alert == other.alert
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LightState {
            #[inline]
            fn clone(&self) -> LightState {
                LightState {
                    on: ::core::clone::Clone::clone(&self.on),
                    brightness: ::core::clone::Clone::clone(&self.brightness),
                    hue: ::core::clone::Clone::clone(&self.hue),
                    saturation: ::core::clone::Clone::clone(&self.saturation),
                    colour_mode: ::core::clone::Clone::clone(&self.colour_mode),
                    colour_temperature: ::core::clone::Clone::clone(&self.colour_temperature),
                    xy: ::core::clone::Clone::clone(&self.xy),
                    mode: ::core::clone::Clone::clone(&self.mode),
                    reachable: ::core::clone::Clone::clone(&self.reachable),
                    alert: ::core::clone::Clone::clone(&self.alert),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for LightState {
            #[inline]
            fn partial_cmp(
                &self,
                other: &LightState,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match :: core :: cmp :: PartialOrd :: partial_cmp (& self . on , & other . on) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . brightness , & other . brightness) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . hue , & other . hue) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . saturation , & other . saturation) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . colour_mode , & other . colour_mode) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . colour_temperature , & other . colour_temperature) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . xy , & other . xy) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . mode , & other . mode) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => match :: core :: cmp :: PartialOrd :: partial_cmp (& self . reachable , & other . reachable) { :: core :: option :: Option :: Some (:: core :: cmp :: Ordering :: Equal) => :: core :: cmp :: PartialOrd :: partial_cmp (& self . alert , & other . alert) , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , } , cmp => cmp , }
            }
        }
        #[allow(clippy::all)]
        ///Builder for [`LightState`](struct.LightState.html).
        pub struct LightStateBuilder {
            on: ::derive_builder::export::core::option::Option<Option<bool>>,
            brightness: ::derive_builder::export::core::option::Option<Option<u32>>,
            hue: ::derive_builder::export::core::option::Option<Option<u32>>,
            saturation: ::derive_builder::export::core::option::Option<Option<u8>>,
            colour_mode: ::derive_builder::export::core::option::Option<Option<String>>,
            colour_temperature: ::derive_builder::export::core::option::Option<Option<u32>>,
            xy: ::derive_builder::export::core::option::Option<Option<XY>>,
            mode: ::derive_builder::export::core::option::Option<Option<String>>,
            reachable: ::derive_builder::export::core::option::Option<Option<bool>>,
            alert: ::derive_builder::export::core::option::Option<Option<String>>,
        }
        #[automatically_derived]
        #[allow(clippy::all)]
        impl ::core::clone::Clone for LightStateBuilder {
            #[inline]
            fn clone(&self) -> LightStateBuilder {
                LightStateBuilder {
                    on: ::core::clone::Clone::clone(&self.on),
                    brightness: ::core::clone::Clone::clone(&self.brightness),
                    hue: ::core::clone::Clone::clone(&self.hue),
                    saturation: ::core::clone::Clone::clone(&self.saturation),
                    colour_mode: ::core::clone::Clone::clone(&self.colour_mode),
                    colour_temperature: ::core::clone::Clone::clone(&self.colour_temperature),
                    xy: ::core::clone::Clone::clone(&self.xy),
                    mode: ::core::clone::Clone::clone(&self.mode),
                    reachable: ::core::clone::Clone::clone(&self.reachable),
                    alert: ::core::clone::Clone::clone(&self.alert),
                }
            }
        }
        #[allow(clippy::all)]
        #[allow(dead_code)]
        impl LightStateBuilder {
            #[allow(unused_mut)]
            pub fn on(&mut self, value: bool) -> &mut Self {
                let mut new = self;
                new.on = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn brightness(&mut self, value: u32) -> &mut Self {
                let mut new = self;
                new.brightness = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn hue(&mut self, value: u32) -> &mut Self {
                let mut new = self;
                new.hue = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn saturation(&mut self, value: u8) -> &mut Self {
                let mut new = self;
                new.saturation = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn colour_mode(&mut self, value: String) -> &mut Self {
                let mut new = self;
                new.colour_mode = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn colour_temperature(&mut self, value: u32) -> &mut Self {
                let mut new = self;
                new.colour_temperature = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn xy(&mut self, value: XY) -> &mut Self {
                let mut new = self;
                new.xy = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn mode(&mut self, value: String) -> &mut Self {
                let mut new = self;
                new.mode = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn reachable(&mut self, value: bool) -> &mut Self {
                let mut new = self;
                new.reachable = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            #[allow(unused_mut)]
            pub fn alert(&mut self, value: String) -> &mut Self {
                let mut new = self;
                new.alert = ::derive_builder::export::core::option::Option::Some(
                    ::derive_builder::export::core::option::Option::Some(value),
                );
                new
            }
            ///Builds a new `LightState`.
            ///
            ///# Errors
            ///
            ///If a required field has not been initialized.
            pub fn build(
                &self,
            ) -> ::derive_builder::export::core::result::Result<LightState, LightStateBuilderError>
            {
                Self::validate(&self)?;
                Ok(LightState {
                    on: match self.on {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("on"),
                                ),
                            )
                        }
                    },
                    brightness: match self.brightness {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("brightness"),
                                ),
                            )
                        }
                    },
                    hue: match self.hue {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("hue"),
                                ),
                            )
                        }
                    },
                    saturation: match self.saturation {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("saturation"),
                                ),
                            )
                        }
                    },
                    colour_mode: match self.colour_mode {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("colour_mode"),
                                ),
                            )
                        }
                    },
                    colour_temperature: match self.colour_temperature {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from(
                                        "colour_temperature",
                                    ),
                                ),
                            )
                        }
                    },
                    xy: match self.xy {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("xy"),
                                ),
                            )
                        }
                    },
                    mode: match self.mode {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("mode"),
                                ),
                            )
                        }
                    },
                    reachable: match self.reachable {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("reachable"),
                                ),
                            )
                        }
                    },
                    alert: match self.alert {
                        Some(ref value) => {
                            ::derive_builder::export::core::clone::Clone::clone(value)
                        }
                        None => {
                            return ::derive_builder::export::core::result::Result::Err(
                                ::derive_builder::export::core::convert::Into::into(
                                    ::derive_builder::UninitializedFieldError::from("alert"),
                                ),
                            )
                        }
                    },
                })
            }
            /// Create an empty builder, with all fields set to `None` or `PhantomData`.
            fn create_empty() -> Self {
                Self {
                    on: ::derive_builder::export::core::default::Default::default(),
                    brightness: ::derive_builder::export::core::default::Default::default(),
                    hue: ::derive_builder::export::core::default::Default::default(),
                    saturation: ::derive_builder::export::core::default::Default::default(),
                    colour_mode: ::derive_builder::export::core::default::Default::default(),
                    colour_temperature: ::derive_builder::export::core::default::Default::default(),
                    xy: ::derive_builder::export::core::default::Default::default(),
                    mode: ::derive_builder::export::core::default::Default::default(),
                    reachable: ::derive_builder::export::core::default::Default::default(),
                    alert: ::derive_builder::export::core::default::Default::default(),
                }
            }
        }
        impl ::derive_builder::export::core::default::Default for LightStateBuilder {
            fn default() -> Self {
                Self::create_empty()
            }
        }
        ///Error type for LightStateBuilder
        #[non_exhaustive]
        pub enum LightStateBuilderError {
            /// Uninitialized field
            UninitializedField(&'static str),
            /// Custom validation error
            ValidationError(::derive_builder::export::core::string::String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LightStateBuilderError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LightStateBuilderError::UninitializedField(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "UninitializedField",
                            &__self_0,
                        )
                    }
                    LightStateBuilderError::ValidationError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ValidationError",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl
            ::derive_builder::export::core::convert::From<::derive_builder::UninitializedFieldError>
            for LightStateBuilderError
        {
            fn from(s: ::derive_builder::UninitializedFieldError) -> Self {
                Self::UninitializedField(s.field_name())
            }
        }
        impl
            ::derive_builder::export::core::convert::From<
                ::derive_builder::export::core::string::String,
            > for LightStateBuilderError
        {
            fn from(s: ::derive_builder::export::core::string::String) -> Self {
                Self::ValidationError(s)
            }
        }
        impl ::derive_builder::export::core::fmt::Display for LightStateBuilderError {
            fn fmt(
                &self,
                f: &mut ::derive_builder::export::core::fmt::Formatter,
            ) -> ::derive_builder::export::core::fmt::Result {
                match self {
                    Self::UninitializedField(ref field) => {
                        f.write_fmt(::core::fmt::Arguments::new_v1(
                            &["`", "` must be initialized"],
                            &[::core::fmt::ArgumentV1::new_display(&field)],
                        ))
                    }
                    Self::ValidationError(ref error) => {
                        f.write_fmt(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&error)],
                        ))
                    }
                }
            }
        }
        impl std::error::Error for LightStateBuilderError {}
        impl LightStateBuilder {
            fn validate(&self) -> Result<(), String> {
                if self.on.is_none() && self.brightness.is_none() {
                    return Err("Either on or brightness must be set".to_string());
                }
                Ok(())
            }
        }
        pub enum LightAlert {
            Select,
            LSelect,
            None,
            Uninitialised,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LightAlert {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LightAlert::Select => ::core::fmt::Formatter::write_str(f, "Select"),
                    LightAlert::LSelect => ::core::fmt::Formatter::write_str(f, "LSelect"),
                    LightAlert::None => ::core::fmt::Formatter::write_str(f, "None"),
                    LightAlert::Uninitialised => {
                        ::core::fmt::Formatter::write_str(f, "Uninitialised")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for LightAlert {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for LightAlert {
            #[inline]
            fn eq(&self, other: &LightAlert) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        pub struct XY {
            x: f32,
            y: f32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for XY {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "XY", "x", &&self.x, "y", &&self.y,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for XY {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for XY {
            #[inline]
            fn eq(&self, other: &XY) -> bool {
                self.x == other.x && self.y == other.y
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for XY {
            #[inline]
            fn partial_cmp(&self, other: &XY) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y)
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for XY {}
        #[automatically_derived]
        impl ::core::clone::Clone for XY {
            #[inline]
            fn clone(&self) -> XY {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        impl<'a> Deserialize<'a> for XY {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'a>,
            {
                let xy: [f32; 2] = Deserialize::deserialize(deserializer)?;
                Ok(XY { x: xy[0], y: xy[1] })
            }
        }
        impl Serialize for XY {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let xy = [self.x, self.y];
                Serialize::serialize(&xy, serializer)
            }
        }
        impl From<&str> for LightAlert {
            fn from(s: &str) -> LightAlert {
                match s {
                    "select" => LightAlert::Select,
                    "lselect" => LightAlert::LSelect,
                    "none" => LightAlert::None,
                    _ => LightAlert::Uninitialised,
                }
            }
        }
        pub struct StateResponse {
            state: LightState,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for StateResponse {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "state" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"state" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<StateResponse>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = StateResponse;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct StateResponse",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                LightState,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct StateResponse with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(StateResponse { state: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<LightState> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "state",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<LightState>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("state") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(StateResponse { state: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["state"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "StateResponse",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<StateResponse>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for StateResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "StateResponse",
                    "state",
                    &&self.state,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for StateResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for StateResponse {
            #[inline]
            fn eq(&self, other: &StateResponse) -> bool {
                self.state == other.state
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for StateResponse {
            #[inline]
            fn clone(&self) -> StateResponse {
                StateResponse {
                    state: ::core::clone::Clone::clone(&self.state),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for StateResponse {
            #[inline]
            fn partial_cmp(
                &self,
                other: &StateResponse,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.state, &other.state)
            }
        }
    }
}
