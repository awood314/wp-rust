#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_extra/juce_gui_extra.h>

#include "PluginProcessor.h"

namespace wprust
{

class WPRustProcessorEditor : public juce::AudioProcessorEditor
{
public:
    WPRustProcessorEditor(WPRustProcessor&);
    ~WPRustProcessorEditor() override;

    void paint(juce::Graphics&) override;
    void resized() override;

private:
    WPRustProcessor& audioProcessor() const;

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(WPRustProcessorEditor)
};

} // namespace wprust
