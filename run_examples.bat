@echo off
echo =======================================
echo Running all fltk-kit examples
echo Make sure you CLOSE each window to run the next one!
echo =======================================

echo [1/9] Basic...
cargo run --example basic

echo [2/9] Layout...
cargo run --example layout

echo [3/9] Form...
cargo run --example form

echo [4/9] Dialogs...
cargo run --example dialogs

echo [5/9] Async Channel...
cargo run --example async_channel

echo [6/9] Tree...
cargo run --example tree

echo [7/9] Table...
cargo run --example table

echo [8/9] FS Watch (Requires feature)...
cargo run --example fs_watch --features fs-watch

echo [9/9] Icon (Requires feature)...
cargo run --example icon --features icon

echo [10/10] Persist (Requires feature)...
cargo run --example persist --features persist

echo All examples completed!
pause