cmake_minimum_required(VERSION 3.0)
set (ENV{PROJECT_NAME} extest)
project($ENV{PROJECT_NAME})

find_package(catkin REQUIRED COMPONENTS
        std_msgs
        message_generation
        )

generate_messages(
        DEPENDENCIES
        std_msgs
)

catkin_package(
        CATKIN_DEPENDS std_msgs message_runtime
)

add_custom_target($ENV{PROJECT_NAME}
        ALL
        COMMAND if [ hash rustc 2>/dev/null ] \; then echo "rustc exists"\; else curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y\; fi\;
        COMMAND echo "==========CMAKE_SOURCE_DIR=========="
        COMMAND echo ${CMAKE_SOURCE_DIR}
        COMMAND echo "==========pwd=========="
        COMMAND pwd
        COMMAND echo "==========ls -al=========="
        COMMAND ls -al
        COMMAND cargo
        COMMAND cd ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release && cargo clean && cargo test --no-run && cargo build --release
        COMMAND echo "==========ls -al ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/=========="
        COMMAND ls -al ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/
        COMMAND echo "==========ls -al ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release=========="
        COMMAND ls -al ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release
        COMMAND echo "==========${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release/$ENV{PROJECT_NAME} ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}=========="
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release/$ENV{PROJECT_NAME} ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}
        COMMAND pwd
        COMMAND find ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/debug/deps -perm /u=x,g=x,o=x -type f | grep deps/extest | xargs -I% cp % ./test
        COMMAND echo "=====hoge====="
        COMMAND ls
        COMMAND echo "=====hoge2====="
        COMMAND pwd
        COMMAND echo "=====hoge2====="
        COMMAND chmod +x ./test
        COMMAND echo "=====hoge2====="
        COMMAND ls -al
        COMMAND echo "=====hoge2====="
        COMMAND ./test
        COMMENT "Building my Rust library"
)

enable_testing()

add_test(NAME $ENV{PROJECT_NAME}_TEST
        COMMAND ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}_test)

unset (ENV{PROJECT_NAME})
