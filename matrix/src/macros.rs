use crate::matrix::Matrix;

/// 矩阵构造宏
#[macro_export]
macro_rules! matrix {
    // 处理带结尾逗号的输入
    ($( [ $($x:expr),* ] ),* $(,)?) => {
        {
            let mut data = Vec::new();
            let mut rows = 0;
            let mut cols = 0;

            $(
                let mut current_cols = 0;
                $(
                    data.push($x);
                    current_cols += 1;
                )*

                // 检查列数一致性
                if rows == 0 {
                    cols = current_cols;
                } else if current_cols != cols {
                    panic!("All rows must have the same number of columns");
                }
                rows += 1;
            )*

            Matrix { rows, cols, data }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_macro() {
        println!("{:?}", matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    }
}
