use cursive::views::Dialog;
use cursive::Cursive;

pub fn dialog_res<T, S: Into<String>>(res: Result<T, S>, s: &mut Cursive) {
    if let Err(err) = res {
        s.add_layer(Dialog::info(err))
    }
}
