#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_extra/juce_gui_extra.h>

namespace wprust {

class Knob : public juce::Component {
public:
  Knob(juce::RangedAudioParameter &param);

  void paint(juce::Graphics &g) override;
  void resized() override;

private:
  juce::Slider _knob;
  juce::SliderParameterAttachment _attachment;
  juce::Label _label;
};

} // namespace wprust