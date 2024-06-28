#include "Knob.h"

namespace wprust {

Knob::Knob(juce::RangedAudioParameter &param) : _attachment(param, _knob) {
  _knob.setSliderStyle(juce::Slider::Rotary);
  _knob.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 100, 20);
  addAndMakeVisible(_knob);

  _label.setText(param.name, juce::sendNotification);
  _label.setJustificationType(juce::Justification::centred);
  addAndMakeVisible(_label);
}

void Knob::paint(juce::Graphics &g) {}

void Knob::resized() {
  juce::FlexBox flexbox;
  flexbox.flexDirection = juce::FlexBox::Direction::column;
  flexbox.justifyContent = juce::FlexBox::JustifyContent::spaceAround;
  flexbox.items.add(juce::FlexItem(_label).withFlex(0.2));
  flexbox.items.add(juce::FlexItem(_knob).withFlex(0.8));
  flexbox.performLayout(getLocalBounds());
}

} // namespace wprust