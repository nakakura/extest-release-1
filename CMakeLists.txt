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
        COMMAND echo "==========ls -al =========="
        COMMAND ls -al
        COMMAND cargo
        COMMAND cd ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release && cargo clean && cargo test --no-run && cargo build --release
        COMMAND find ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/debug/deps -perm /u=x,g=x,o=x -type f | xargs -I% mv % ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}_test
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release/$ENV{PROJECT_NAME} ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}
        COMMAND rm ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}-release/target/release/$ENV{PROJECT_NAME}
        COMMENT "Building my Rust library"
)

enable_testing()

add_test(NAME $ENV{PROJECT_NAME}_TEST
        COMMAND ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}_test)

unset (ENV{PROJECT_NAME})
