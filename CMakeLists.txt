cmake_minimum_required(VERSION 3.0)
# add_custom_targetでコマンドを叩くので環境変数として定義しておく
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
        COMMAND echo ${CMAKE_SOURCE_DIR}
        COMMAND if [ hash rustc 2>/dev/null ] \; then echo "rustc exists"\; else curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y\; fi\;
        COMMAND cd ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME} && cargo clean && cargo test --no-run && cargo build --release
        COMMAND find ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}/target/debug/deps -perm /u=x,g=x,o=x -type f | xargs -I% mv % ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}_test
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}/target/release/$ENV{PROJECT_NAME} ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}
        COMMAND rm ${CMAKE_SOURCE_DIR}/$ENV{PROJECT_NAME}/target/release/$ENV{PROJECT_NAME}
        COMMENT "Building my Rust library"
)

enable_testing()

add_test(NAME $ENV{PROJECT_NAME}_TEST
        COMMAND ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/$ENV{PROJECT_NAME}_test)

# セットした環境変数をクリアしておく
unset (ENV{PROJECT_NAME})
