/// [735. 小行星碰撞](https://leetcode.cn/problems/asteroid-collision/)
pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let n = asteroids.len();
        let mut stack = Vec::with_capacity(n);
        for aster in asteroids {
            let mut alive = true; // 当前处理的小行星还存活
            if aster < 0 {
                while let Some(&prev) = stack.last() {
                    if prev > 0 {
                        // 防止aster == i32::MIN时取绝对值溢出：
                        if prev + aster == 0 {
                            stack.pop();
                            alive = false;
                            break;
                        } else if prev + aster > 0 {
                            alive = false;
                            break;
                        } else {
                            stack.pop();
                        }
                    } else {
                        break;
                    }
                }
            }

            if alive {
                stack.push(aster);
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
