pub mod app;
pub mod models;

#[cfg(test)]
pub mod tests {
    pub mod gui_tests;
    pub mod window_title_tests;
    pub mod plugin_tests;
    pub mod vcvrack_app_tests;
    pub mod startup_tests;
}
