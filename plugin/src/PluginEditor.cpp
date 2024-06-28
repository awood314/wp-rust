#include <juce_core/juce_core.h>

#include "PluginEditor.h"
#include "PluginProcessor.h"

#include <cassert>

namespace wprust {

//==============================================================================
WPRustProcessorEditor::WPRustProcessorEditor(WPRustProcessor &p)
    : AudioProcessorEditor(&p), _saturation(p.saturationParam),
      _frequency(p.frequencyParam), _rate(p.rateParam), _depth(p.depthParam),
      _feedback(p.feedbackParam) {

  for (auto *knob : {&_saturation, &_frequency, &_rate, &_depth, &_feedback}) {
    addAndMakeVisible(*knob);
  }

  setResizable(true, true);
  setSize(600, 200);
}

WPRustProcessorEditor::~WPRustProcessorEditor() {}

void WPRustProcessorEditor::paint(juce::Graphics &) {}

void WPRustProcessorEditor::resized() {
  juce::FlexBox flexbox;
  flexbox.flexDirection = juce::FlexBox::Direction::row;
  flexbox.justifyContent = juce::FlexBox::JustifyContent::spaceAround;
  for (auto *knob : {&_saturation, &_frequency, &_rate, &_depth, &_feedback}) {
    flexbox.items.add(juce::FlexItem(*knob).withFlex(1.0f));
  }

  flexbox.performLayout(getLocalBounds());
}

WPRustProcessor &WPRustProcessorEditor::audioProcessor() const {
  assert(dynamic_cast<WPRustProcessor *>(getAudioProcessor()));
  return *static_cast<WPRustProcessor *>(getAudioProcessor());
}

} // namespace wprust
