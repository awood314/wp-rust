#pragma once

#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_extra/juce_gui_extra.h>

#include "PluginProcessor.h"

namespace wprust {

class WPRustProcessorEditor : public juce::AudioProcessorEditor {
public:
  WPRustProcessorEditor(WPRustProcessor &);
  ~WPRustProcessorEditor() override;

  void paint(juce::Graphics &) override;
  void resized() override;

private:
  WPRustProcessor &audioProcessor() const;

  juce::Slider _frequencyKnob;
  juce::SliderParameterAttachment _frequencyAttachment;

  juce::Slider _rateKnob;
  juce::SliderParameterAttachment _rateAttachment;
  juce::Slider _depthKnob;
  juce::SliderParameterAttachment _depthAttachment;
  juce::Slider _feedbackKnob;
  juce::SliderParameterAttachment _feedbackAttachment;

  JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(WPRustProcessorEditor)
};

} // namespace wprust
