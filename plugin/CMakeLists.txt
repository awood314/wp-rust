CMAKE_MINIMUM_REQUIRED(VERSION 3.15)
PROJECT(WPRust VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(
    "${CMAKE_CURRENT_SOURCE_DIR}/juce" "${CMAKE_CURRENT_BINARY_DIR}/juce" EXCLUDE_FROM_ALL)

juce_add_plugin(${PROJECT_NAME}
    PLUGIN_NAME ${PROJECT_NAME}
    PRODUCT_NAME ${PROJECT_NAME}
    BUNDLE_ID "com.awood314.wp-rust"
    COMPANY_NAME ${PROJECT_NAME}
    COMPANY_WEBSITE "https://github.com/awood314"
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    VST3_CATEGORIES "Fx"
    AU_MAIN_TYPE kAudioUnitType_MusicDevice
    PLUGIN_MANUFACTURER_CODE Awoo
    PLUGIN_CODE Aaaa
    FORMATS VST3 AU
    COPY_PLUGIN_AFTER_BUILD TRUE)

set(TARGET_SOURCES
    src/PluginProcessor.cpp
    src/PluginProcessor.h
    src/PluginEditor.cpp
    src/PluginEditor.h)

target_sources(${PROJECT_NAME} PRIVATE ${TARGET_SOURCES})

set(COMPILE_DEFINITIONS
    JUCE_USE_CURL=0
    JUCE_WEB_BROWSER=0
    JUCE_REPORT_APP_USAGE=0
    JUCE_DISPLAY_SPLASH_SCREEN=0
    JUCE_VST3_CAN_REPLACE_VST2=0)

target_compile_definitions(${PROJECT_NAME} PRIVATE ${COMPILE_DEFINITIONS})

set(LINK_LIBRARIES
    juce::juce_audio_formats
    juce::juce_audio_plugin_client
    juce::juce_audio_utils
    juce::juce_gui_basics)

target_link_libraries(${PROJECT_NAME} PRIVATE ${LINK_LIBRARIES})

if(MSVC)
  target_compile_options(${PROJECT_NAME} PRIVATE /W4 /MP)
else()
  target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra -Wpedantic)
  add_link_options(/incremental /Debug:fastlink)

  set(CMAKE_CXX_FLAGS_RELEASE "-DNDEBUG -Ofast")
  set(CMAKE_C_FLAGS_RELEASE "-DNDEBUG -Ofast")
endif()
