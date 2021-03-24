#[macro_export]
macro_rules! pt {
    [ $func:ident($($args:expr),*) $($other:tt)+] => {{
        let mut temp = $func($($args),*);
        pt![temp $($other)+]
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))?. $($other:tt)+] => {{
        let mut temp = &mut $a.$b$(($($args),*))?;
        ref_pt![temp . $($other)+]
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))?-> $($other:tt)+] => {{
        let mut temp = &mut $a.$b$(($($args),*))?;
        ref_pt![temp -> $($other)+]
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))?. $($other:tt)+] => {{
        let mut temp = &mut (*$a).$b$(($($args),*))?;
        ref_pt![temp . $($other)+]
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))?-> $($other:tt)+] => {{
        let mut temp = &mut (*$a).$b$(($($args),*))?;
        ref_pt![temp -> $($other)+]
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))? ] => {{
        &mut $a.$b$(($($args),*))?
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? ] => {{
        &mut (*$a).$b$(($($args),*))?
    }};

    [ $a:ident . $b:ident $(($($args:expr),*))? = $c:ident $($other:tt)+] => {{
        $a.$b$(($($args),*))? = *pt![$c $($other)+];
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? = $c:ident $($other:tt)+] => {{
        (*$a).$b$(($($args),*))? = *pt![$c $($other)+];
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))? = $c:expr] => {{
        $a.$b$(($($args),*))? = $c;
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? = $c:expr] => {{
        (*$a).$b$(($($args),*))? = $c;
    }};
}

#[macro_export]
macro_rules! ref_pt {
    [ $a:ident . $b:ident $(($($args:expr),*))?. $($other:tt)+] => {{
        let mut temp = &mut (*$a).$b$(($($args),*))?;
        ref_pt![temp . $($other)+]
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))?-> $($other:tt)+] => {{
        let mut temp = &mut (*$a).$b$(($($args),*))?;
        ref_pt![temp -> $($other)+]
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))?. $($other:tt)+] => {{
        let mut temp = &mut (**$a).$b$(($($args),*))?;
        ref_pt![temp . $($other)+]
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))?-> $($other:tt)+] => {{
        let mut temp = &mut (**$a).$b$(($($args),*))?;
        ref_pt![temp -> $($other)+]
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))? ] => {{
        &mut (*$a).$b$(($($args),*))?
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? ] => {{
        &mut (**$a).$b$(($($args),*))?
    }};

    [ $a:ident . $b:ident $(($($args:expr),*))? = $c:ident $($other:tt)+] => {{
        (*$a).$b$(($($args),*))? = *pt![$c $($other)+];
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? = $c:ident $($other:tt)+] => {{
        (**$a).$b$(($($args),*))? = *pt![$c $($other)+];
    }};
    [ $a:ident . $b:ident $(($($args:expr),*))? = $c:expr] => {{
        (*$a).$b$(($($args),*))? = $c;
    }};
    [ $a:ident -> $b:ident $(($($args:expr),*))? = $c:expr] => {{
        (**$a).$b$(($($args),*))? = $c;
    }};
}
