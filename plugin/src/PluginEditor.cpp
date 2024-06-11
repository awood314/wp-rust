#include <juce_core/juce_core.h>

#include "PluginEditor.h"
#include "PluginProcessor.h"

#include <cassert>

namespace wprust {

//==============================================================================
WPRustProcessorEditor::WPRustProcessorEditor(WPRustProcessor &p)
    : AudioProcessorEditor(&p),
      _frequencyAttachment(p.frequencyParam, _frequencyKnob) {
  _frequencyKnob.setSliderStyle(juce::Slider::Rotary);
  _frequencyKnob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  addAndMakeVisible(_frequencyKnob);

  setResizable(true, true);
  setSize(500, 500);
}

WPRustProcessorEditor::~WPRustProcessorEditor() {}

void WPRustProcessorEditor::paint(juce::Graphics &) {}

void WPRustProcessorEditor::resized() {
  _frequencyKnob.setBounds(getLocalBounds().reduced(40));
}

WPRustProcessor &WPRustProcessorEditor::audioProcessor() const {
  assert(dynamic_cast<WPRustProcessor *>(getAudioProcessor()));
  return *static_cast<WPRustProcessor *>(getAudioProcessor());
}

} // namespace wprust
