#include <juce_core/juce_core.h>

#include "PluginEditor.h"
#include "PluginProcessor.h"
#include "iir_filter.rs.h"
#include "waveshaper.rs.h"

namespace wprust {

WPRustProcessor::WPRustProcessor()
#ifndef JucePlugin_PreferredChannelConfigurations
    : AudioProcessor(
          BusesProperties()
#if !JucePlugin_IsMidiEffect
#if !JucePlugin_IsSynth
              .withInput("Input", juce::AudioChannelSet::stereo(), true)
#endif
              .withOutput("Output", juce::AudioChannelSet::stereo(), true)
#endif
      )
#endif
{
}

WPRustProcessor::~WPRustProcessor() {}

const juce::String WPRustProcessor::getName() const { return JucePlugin_Name; }

bool WPRustProcessor::acceptsMidi() const {
#if JucePlugin_WantsMidiInput
  return true;
#else
  return false;
#endif
}

bool WPRustProcessor::producesMidi() const {
#if JucePlugin_ProducesMidiOutput
  return true;
#else
  return false;
#endif
}

bool WPRustProcessor::isMidiEffect() const {
#if JucePlugin_IsMidiEffect
  return true;
#else
  return false;
#endif
}

double WPRustProcessor::getTailLengthSeconds() const { return 0.0; }

int WPRustProcessor::getNumPrograms() {
  return 1; // NB: some hosts don't cope very well if you tell them there are 0
            // programs, so this should be at least 1, even if you're not really
            // implementing programs.
}

int WPRustProcessor::getCurrentProgram() { return 0; }

void WPRustProcessor::setCurrentProgram(int) {}

const juce::String WPRustProcessor::getProgramName(int) { return {}; }

void WPRustProcessor::changeProgramName(int, const juce::String &) {}

void WPRustProcessor::prepareToPlay(double sampleRate, int) {
  rust::iir::reset(_filters[0], sampleRate);
  rust::iir::reset(_filters[1], sampleRate);

  _modDelay[0]->reset(sampleRate);
  _modDelay[1]->reset(sampleRate);
}

void WPRustProcessor::releaseResources() {
  // When playback stops, you can use this as an opportunity to free up any
  // spare memory, etc.
}

#ifndef JucePlugin_PreferredChannelConfigurations
bool WPRustProcessor::isBusesLayoutSupported(const BusesLayout &layouts) const {
#if JucePlugin_IsMidiEffect
  juce::ignoreUnused(layouts);
  return true;
#else
  // This is the place where you check if the layout is supported.
  // In this template code we only support mono or stereo.
  // Some plugin hosts, such as certain GarageBand versions, will only
  // load plugins that support stereo bus layouts.
  if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::mono() &&
      layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo())
    return false;

    // This checks if the input layout matches the output layout
#if !JucePlugin_IsSynth
  if (layouts.getMainOutputChannelSet() != layouts.getMainInputChannelSet())
    return false;
#endif

  return true;
#endif
}
#endif

void WPRustProcessor::processBlock(juce::AudioBuffer<float> &audioBuffer,
                                   juce::MidiBuffer &) {
  juce::ScopedNoDenormals noDenormals;

  // process
  for (int i = 0; i < audioBuffer.getNumChannels(); i++) {
    if (i < 2) {
      // set parameters
      rust::iir::set_parameters(_filters[i], {frequencyParam.get()});
      _modDelay[i]->set_parameters({.depth_pct = depthParam.get(),
                                    .feedback = feedbackParam.get(),
                                    .rate = rateParam.get()});

      const auto &channelRead = audioBuffer.getReadPointer(i);
      const auto &channelWrite = audioBuffer.getWritePointer(i);
      for (int j = 0; j < audioBuffer.getNumSamples(); j++) {
        auto xn = channelRead[j];

        // Waveshaper
        xn = rust::waveshaper::process(xn, rust::waveshaper::Function::HypTan,
                                       saturationParam.get() * 5);

        // Modulated Delay
        xn = _modDelay[i]->process(xn);

        // LPF
        xn = rust::iir::process(_filters[i], xn);

        channelWrite[j] = xn;
      }
    }
  }
}

//==============================================================================
bool WPRustProcessor::hasEditor() const {
  return true; // (change this to false if you choose to not supply an editor)
}

juce::AudioProcessorEditor *WPRustProcessor::createEditor() {
  return new WPRustProcessorEditor(*this);
}

//==============================================================================
void WPRustProcessor::getStateInformation(juce::MemoryBlock &) {}

void WPRustProcessor::setStateInformation(const void *, int) {}

} // namespace wprust

// This creates new instances of the plugin..
juce::AudioProcessor *JUCE_CALLTYPE createPluginFilter() {
  return new wprust::WPRustProcessor();
}
