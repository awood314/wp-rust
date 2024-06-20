#include <juce_core/juce_core.h>

#include "PluginEditor.h"
#include "PluginProcessor.h"

#include <cassert>

namespace wprust {

//==============================================================================
WPRustProcessorEditor::WPRustProcessorEditor(WPRustProcessor &p)
    : AudioProcessorEditor(&p),
      _frequencyAttachment(p.frequencyParam, _frequencyKnob),
      _rateAttachment(p.rateParam, _rateKnob),
      _depthAttachment(p.depthParam, _depthKnob),
      _feedbackAttachment(p.feedbackParam, _feedbackKnob) {
  // _frequencyKnob.setSliderStyle(juce::Slider::Rotary);
  // _frequencyKnob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  // addAndMakeVisible(_frequencyKnob);

  _rateKnob.setSliderStyle(juce::Slider::Rotary);
  _rateKnob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  addAndMakeVisible(_rateKnob);

  _depthKnob.setSliderStyle(juce::Slider::Rotary);
  _depthKnob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  addAndMakeVisible(_depthKnob);

  _feedbackKnob.setSliderStyle(juce::Slider::Rotary);
  _feedbackKnob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  addAndMakeVisible(_feedbackKnob);

  setResizable(true, true);
  setSize(600, 200);
}

WPRustProcessorEditor::~WPRustProcessorEditor() {}

void WPRustProcessorEditor::paint(juce::Graphics &) {}

void WPRustProcessorEditor::resized() {
  juce::FlexBox flexbox;
  flexbox.flexDirection = juce::FlexBox::Direction::row;
  flexbox.justifyContent = juce::FlexBox::JustifyContent::spaceAround;

  flexbox.items.add(juce::FlexItem(_rateKnob).withFlex(1.0f));
  flexbox.items.add(juce::FlexItem(_depthKnob).withFlex(1.0f));
  flexbox.items.add(juce::FlexItem(_feedbackKnob).withFlex(1.0f));

  flexbox.performLayout(getLocalBounds());
}

WPRustProcessor &WPRustProcessorEditor::audioProcessor() const {
  assert(dynamic_cast<WPRustProcessor *>(getAudioProcessor()));
  return *static_cast<WPRustProcessor *>(getAudioProcessor());
}

} // namespace wprust
