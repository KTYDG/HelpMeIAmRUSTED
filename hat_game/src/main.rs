fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // other сохраняет значение
        _ => reroll(), // _ не сохраняет значение и тут вызывается функция переброса
        _ => (),  // _ не сохраняет значение и никакая функция не вызывается
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}
