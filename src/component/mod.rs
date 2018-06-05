use super::canvas::operation::operation_set::OperationSet;

#[macro_export]
macro_rules! render {
    ( $e:expr ) => {
        $e
    };
    ( $( $e:expr ),+ ) => {
        {
            let mut temp_opp = OperationSet::new();
            $(
                temp_opp.push($e),
            )*
            temp_opp
        }
    };
}
