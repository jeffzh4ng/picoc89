use crate::{IBinOp, IExpr, IPrg, IStmt, Label, SBinOp, SDef, SExpr, SFuncDef, SPrg, SStmt, Temp};

pub fn translate(src_tree: &SPrg) -> IPrg {
    let intrm_prg = src_tree
        .iter()
        .map(|def| match def {
            SDef::FuncDef(func_def) => translate_func_def(func_def),
            SDef::VarDef(_var_def) => todo!(),
        })
        .collect::<Vec<_>>();

    intrm_prg
}

fn translate_func_def(fd: &SFuncDef) -> IStmt {
    let label = Label::UserLabel(fd.alias.clone());

    // todo: formal params
    let body = fd
        .body
        .iter()
        .map(|s_stmt| match s_stmt {
            SStmt::Asnmt(vd) => {
                let expr = translate_expr(&vd.expr);
                let temp = Temp::UserTemp(vd.alias.clone());
                IStmt::Compute(temp, expr)
                // ************************************* ??????????????zsd
            }
            SStmt::IfEls {
                cond: _,
                then: _,
                els: _,
            } => todo!(),
            SStmt::While { cond: _, body: _ } => todo!(),
            SStmt::Return(expr) => IStmt::Return(translate_expr(expr)),
        })
        .map(Box::new)
        .collect::<Vec<_>>();

    IStmt::Seq(label, body)
}

fn translate_expr(e: &SExpr) -> IExpr {
    match e {
        SExpr::Int(n) => IExpr::Const(*n),
        SExpr::Bool(b) => IExpr::Const(*b as i32),
        SExpr::UnaryE { op: _, l: _ } => todo!(),
        SExpr::BinE { op, l, r } => match op {
            // C language designed as portable assembly makes tree rewrites straightforward
            SBinOp::Add => IExpr::BinOp(
                IBinOp::Add,
                Box::new(translate_expr(l)),
                Box::new(translate_expr(r)),
            ),
            SBinOp::Sub => IExpr::BinOp(
                IBinOp::Sub,
                Box::new(translate_expr(l)),
                Box::new(translate_expr(r)),
            ),
            SBinOp::Mult => IExpr::BinOp(
                IBinOp::Mult,
                Box::new(translate_expr(l)),
                Box::new(translate_expr(r)),
            ),
            SBinOp::Div => IExpr::BinOp(
                IBinOp::Div,
                Box::new(translate_expr(l)),
                Box::new(translate_expr(r)),
            ),
            SBinOp::Mod => IExpr::BinOp(
                IBinOp::Mod,
                Box::new(translate_expr(l)),
                Box::new(translate_expr(r)),
            ),
        },
        SExpr::LogE { op: _, l: _, r: _ } => todo!(),
        SExpr::BitE { op: _, l: _, r: _ } => todo!(),
        SExpr::RelE { op: _, l: _, r: _ } => todo!(),
        SExpr::VarApp(alias) => IExpr::TempUse(Temp::UserTemp(alias.clone())),
        SExpr::FuncApp { alias, aps: ap } => {
            let aps = ap.iter().map(translate_expr).collect::<Vec<_>>();
            IExpr::Call(Label::UserLabel(alias.clone()), aps)
        }
    }
}

#[cfg(test)]
mod test_arith {
    use crate::lexer;
    use crate::parser_ast;
    use crate::typer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/arith";

    #[test]
    fn add() {
        let chars = fs::read(format!("{TEST_DIR}/add.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let src_tree = parser_ast::parse_prg(&tokens).unwrap();
        let _ = typer::type_prg(&src_tree).unwrap();
        let trgt_tree = super::translate(&src_tree);

        insta::assert_yaml_snapshot!(trgt_tree, @r###"
        ---
        - Seq:
            - UserLabel: main
            - - Return:
                  BinOp:
                    - Add
                    - Const: 9
                    - Const: 10
        "###);
    }
}

#[cfg(test)]
mod test_bindings {
    use crate::lexer;
    use crate::parser_ast;
    use crate::typer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";

    #[test]
    fn asnmt() {
        let chars = fs::read(format!("{TEST_DIR}/asnmt.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let src_tree = parser_ast::parse_prg(&tokens).unwrap();
        let _ = typer::type_prg(&src_tree).unwrap();
        let trgt_tree = super::translate(&src_tree);

        insta::assert_yaml_snapshot!(trgt_tree, @r###"
        ---
        - Seq:
            - UserLabel: main
            - - Compute:
                  - UserTemp: x
                  - Const: 8
              - Return:
                  TempUse:
                    UserTemp: x
        "###);
    }
}

#[cfg(test)]
mod test_functions {
    use crate::lexer;
    use crate::parser_ast;
    use crate::typer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";

    #[test]
    fn composition() {
        let chars = fs::read(format!("{TEST_DIR}/composition.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let src_tree = parser_ast::parse_prg(&tokens).unwrap();
        let _ = typer::type_prg(&src_tree).unwrap();
        let trgt_tree = super::translate(&src_tree);

        insta::assert_yaml_snapshot!(trgt_tree, @r###"
        ---
        - Seq:
            - UserLabel: h
            - - Return:
                  Const: 11
        - Seq:
            - UserLabel: g
            - - Return:
                  BinOp:
                    - Add
                    - Const: 10
                    - Call:
                        - UserLabel: h
                        - []
        - Seq:
            - UserLabel: f
            - - Return:
                  BinOp:
                    - Add
                    - Const: 9
                    - Call:
                        - UserLabel: g
                        - []
        - Seq:
            - UserLabel: main
            - - Return:
                  Call:
                    - UserLabel: f
                    - []
        "###);
    }

    #[test]
    fn formal_param() {
        let chars = fs::read(format!("{TEST_DIR}/formal_param.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let src_tree = parser_ast::parse_prg(&tokens).unwrap();
        let _ = typer::type_prg(&src_tree).unwrap();
        let trgt_tree = super::translate(&src_tree);

        insta::assert_yaml_snapshot!(trgt_tree, @r"");
    }
}
