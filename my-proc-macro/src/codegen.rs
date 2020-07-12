// use proc_macro2::TokenStream as TokenStream;
// use quote::quote;

// pub fn app(app: &App, analysis: &Analysis, extra: &Extra) -> TokenStream {
//     let mut const_app = vec![];
//     let mut mains = vec![];
//     let mut root = vec![];
//     let mut user = vec![];

//     // generate a `main` function for each core
//     for core in 0..app.args.cores {
//         let assertion_stmts = assertions::codegen(core, analysis, extra);

//         let (const_app_pre_init, pre_init_stmts) = pre_init::codegen(core, &app, analysis, extra);

//         let (const_app_init, root_init, user_init, call_init) =
//             init::codegen(core, app, analysis, extra);

//         let (const_app_post_init, post_init_stmts) =
//             post_init::codegen(core, &app, analysis, extra);

//         let (const_app_idle, root_idle, user_idle, call_idle) =
//             idle::codegen(core, app, analysis, extra);

//         user.push(quote!(
//             #user_init

//             #user_idle
//         ));

//         root.push(quote!(
//             #(#root_init)*

//             #(#root_idle)*
//         ));

//         const_app.push(quote!(
//             #(#const_app_pre_init)*

//             #const_app_init

//             #(#const_app_post_init)*

//             #const_app_idle
//         ));

//         let cfg_core = util::cfg_core(core, app.args.cores);
//         let main = util::suffixed("main", core);
//         let section = util::link_section("text", core);
//         mains.push(quote!(
//             #[no_mangle]
//             #section
//             #cfg_core
//             unsafe extern "C" fn #main() -> ! {
//                 let _TODO: () = ();

//                 #(#assertion_stmts)*

//                 #(#pre_init_stmts)*

//                 #call_init

//                 #(#post_init_stmts)*

//                 #call_idle
//             }
//         ));
//     }

//     let (const_app_resources, mod_resources) = resources::codegen(app, analysis, extra);

//     let (const_app_hardware_tasks, root_hardware_tasks, user_hardware_tasks) =
//         hardware_tasks::codegen(app, analysis, extra);

//     let (const_app_software_tasks, root_software_tasks, user_software_tasks) =
//         software_tasks::codegen(app, analysis, extra);

//     let const_app_dispatchers = dispatchers::codegen(app, analysis, extra);

//     let const_app_spawn = spawn::codegen(app, analysis, extra);

//     let const_app_timer_queue = timer_queue::codegen(app, analysis, extra);

//     let const_app_schedule = schedule::codegen(app, extra);

//     let cores = app.args.cores.to_string();
//     let cfg_core = quote!(#[cfg(core = #cores)]);
//     let msg = format!(
//         "specified {} core{} but tried to compile for more than {0} core{1}",
//         app.args.cores,
//         if app.args.cores > 1 { "s" } else { "" }
//     );
//     let check_excess_cores = quote!(
//         #cfg_core
//         compile_error!(#msg);
//     );

//     let name = &app.name;
//     let device = extra.device;
//     quote!(

//     )
// }
