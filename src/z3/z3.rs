use std::ffi::CString;
use std::string::String;
use z3_sys;

pub enum SMTResults {
    SAT,
    UNSAT,
    UNDEF,
}

pub fn solve(filename: String) -> SMTResults {
    let mut result = SMTResults::SAT;

    unsafe {
        let cstr = CString::new(filename).unwrap();
        let config = z3_sys::Z3_mk_config();
        let ctx = z3_sys::Z3_mk_context(config);
        let solver = z3_sys::Z3_mk_solver(ctx);

        z3_sys::Z3_solver_from_file(ctx, solver, cstr.as_ptr());

        let is_sat = z3_sys::Z3_solver_check(ctx, solver);
        if is_sat == z3_sys::Z3_L_FALSE {
            result = SMTResults::UNSAT;
        } else if is_sat == z3_sys::Z3_L_UNDEF {
            result = SMTResults::UNDEF;
        }
    }

    result
}
