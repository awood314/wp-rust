#pragma once

#include "Knob.h"
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

  Knob _saturation;
  Knob _frequency;
  Knob _rate;
  Knob _depth;
  Knob _feedback;

  JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(WPRustProcessorEditor)
};

} // namespace wprust
