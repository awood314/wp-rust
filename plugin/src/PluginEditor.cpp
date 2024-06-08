#include <juce_core/juce_core.h>

#include "PluginEditor.h"
#include "PluginProcessor.h"

#include <cassert>

namespace wprust
{

//==============================================================================
WPRustProcessorEditor::WPRustProcessorEditor(WPRustProcessor& p)
: AudioProcessorEditor(&p)
{
    setResizable(true, true);
    setSize(500, 500);
}

WPRustProcessorEditor::~WPRustProcessorEditor()
{}

void WPRustProcessorEditor::paint(juce::Graphics&)
{}

void WPRustProcessorEditor::resized()
{}

WPRustProcessor& WPRustProcessorEditor::audioProcessor() const
{
    assert(dynamic_cast<WPRustProcessor*>(getAudioProcessor()));
    return *static_cast<WPRustProcessor*>(getAudioProcessor());
}

} // namespace wprust
