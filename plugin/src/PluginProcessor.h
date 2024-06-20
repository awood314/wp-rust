#pragma once

#pragma GCC diagnostic ignored "-Wdollar-in-identifier-extension"
#include "iir_filter.rs.h"
#include "mod_delay.rs.h"

#include <juce_audio_basics/juce_audio_basics.h>
#include <juce_audio_processors/juce_audio_processors.h>

namespace wprust {

class WPRustProcessor : public juce::AudioProcessor {
public:
  WPRustProcessor();
  ~WPRustProcessor() override;

  void prepareToPlay(double sampleRate, int samplesPerBlock) override;
  void releaseResources() override;

#ifndef JucePlugin_PreferredChannelConfigurations
  bool isBusesLayoutSupported(const BusesLayout &layouts) const override;
#endif

  void processBlock(juce::AudioBuffer<float> &, juce::MidiBuffer &) override;

  juce::AudioProcessorEditor *createEditor() override;
  bool hasEditor() const override;

  const juce::String getName() const override;

  bool acceptsMidi() const override;
  bool producesMidi() const override;
  bool isMidiEffect() const override;
  double getTailLengthSeconds() const override;

  int getNumPrograms() override;
  int getCurrentProgram() override;
  void setCurrentProgram(int index) override;
  const juce::String getProgramName(int index) override;
  void changeProgramName(int index, const juce::String &newName) override;

  void getStateInformation(juce::MemoryBlock &destData) override;
  void setStateInformation(const void *data, int sizeInBytes) override;

  juce::AudioParameterFloat frequencyParam{
      "frequency", "Frequency", {20.f, 20480.f}, 1000.f};

  juce::AudioParameterFloat rateParam{"rate", "Rate", {0.02f, 20.f}, 0.2f};
  juce::AudioParameterFloat depthParam{"depth", "Depth", {0, 100}, 50};
  juce::AudioParameterFloat feedbackParam{"feedback", "Feedback", {0, 100}, 50};

private:
  rust::iir::AudioFilter _filters[2];

  ::rust::Box<rust::mod::ModulatedDelay> _modDelay[2] = {
      rust::mod::create_modulated_delay(), rust::mod::create_modulated_delay()};

  JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(WPRustProcessor)
};

} // namespace wprust
