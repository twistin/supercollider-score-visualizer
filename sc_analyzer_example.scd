// sc_analyzer_example.scd
// SuperCollider script to test /analyzer OSC messages with SC Score Visualizer

(
// Configure OSC target
var visualizer = NetAddr("127.0.0.1", 7777);

// Test /analyzer message with proper format
// Format: /analyzer timestamp pitch amplitude onset duration timbre_features
~sendAnalyzerEvent = {
    arg timestamp=1.0, pitch=440.0, amplitude=0.8, onset=0.1, duration=2.0, timbre=0.6;
    
    visualizer.sendMsg("/analyzer", timestamp, pitch, amplitude, onset, duration, timbre);
    ("Sent /analyzer message: timestamp=" ++ timestamp ++ 
     " pitch=" ++ pitch ++ "Hz amplitude=" ++ amplitude ++ 
     " onset=" ++ onset ++ " duration=" ++ duration ++ 
     " timbre=" ++ timbre).postln;
};

// Test different types of events
"Testing /analyzer OSC messages...".postln;

// Test 1: Basic note
~sendAnalyzerEvent.(1.0, 440.0, 0.8, 0.1, 2.0, 0.3); // A4, low timbre -> Note
wait(0.5);

// Test 2: Higher pitch, more complex timbre
~sendAnalyzerEvent.(1.5, 880.0, 0.6, 0.05, 1.5, 0.6); // A5, mid timbre -> Chord  
wait(0.5);

// Test 3: Very high pitch, complex timbre
~sendAnalyzerEvent.(2.0, 1760.0, 0.9, 0.02, 0.8, 0.9); // A6, high timbre -> Transient
wait(0.5);

// Test 4: Low bass note
~sendAnalyzerEvent.(2.5, 110.0, 0.7, 0.2, 3.0, 0.2); // A2, low timbre -> Note
wait(0.5);

// Test 5: Mid-range with varying parameters
~sendAnalyzerEvent.(3.0, 523.25, 0.75, 0.08, 1.2, 0.5); // C5, mid timbre -> Chord

"Finished sending test messages. Check the visualizer!".postln;
)

// Advanced test with real-time audio analysis
(
// Setup audio analysis for real-time /analyzer messages
var fft, analyzeFunc;
var netAddr = NetAddr("127.0.0.1", 7777);

// Start audio analysis
fft = FFT(LocalBuf(1024), SoundIn.ar(0));

// Function to analyze audio and send to visualizer  
analyzeFunc = {
    var signal = SoundIn.ar(0);
    var amplitude = Amplitude.kr(signal, 0.01, 0.1);
    var pitch = Pitch.kr(signal, ampThreshold: 0.02, median: 1);
    var onset = Onsets.kr(fft, 0.3, \rcomplex);
    var spectralCentroid = SpectralCentroid.kr(fft);
    var timbre = spectralCentroid.linlin(0, 8000, 0.0, 1.0);
    
    // Send analyzer message when there's significant audio
    SendReply.kr(amplitude > 0.05, "/analyzer", [
        Main.elapsedTime, // timestamp
        pitch[0],         // pitch
        amplitude,        // amplitude
        onset,           // onset
        2.0,             // duration (fixed for now)
        timbre           // timbre features
    ]);
};

// Start real-time analysis
~analysisNode = { analyzeFunc.value }.play;

// Setup OSC responder to forward to visualizer
~oscResponder = OSCFunc({ |msg, time, addr, recvPort|
    if (msg.size >= 7) {
        netAddr.sendMsg("/analyzer", 
            msg[3], // timestamp
            msg[4], // pitch  
            msg[5], // amplitude
            msg[6], // onset
            msg[7], // duration
            msg[8]  // timbre
        );
        ("Real-time /analyzer: pitch=" ++ msg[4].round(0.1) ++ 
         "Hz amp=" ++ msg[5].round(0.01) ++ 
         " timbre=" ++ msg[8].round(0.01)).postln;
    };
}, "/analyzer");

"Real-time audio analysis started. Make some sound!".postln;
"Run ~analysisNode.free; ~oscResponder.free; to stop.".postln;
)

// Stop real-time analysis
(
~analysisNode.free;
~oscResponder.free;
"Real-time analysis stopped.".postln;
)

// Test sequence with musical phrases
(
var visualizer = NetAddr("127.0.0.1", 7777);
var sendAnalyzer = { |timestamp, pitch, amplitude, onset, duration, timbre|
    visualizer.sendMsg("/analyzer", timestamp, pitch, amplitude, onset, duration, timbre);
};

// C major scale with varying timbres
var notes = [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25]; // C4 to C5
var currentTime = 0;

"Playing C major scale with /analyzer messages...".postln;

notes.do({ |freq, i|
    var amplitude = 0.3 + (0.4 * (i/7)); // Crescendo
    var timbre = i / 7.0; // Increasing timbral complexity
    var duration = 0.8;
    
    sendAnalyzer.(currentTime, freq, amplitude, 0.05, duration, timbre);
    currentTime = currentTime + 0.6;
});

"Scale sequence sent!".postln;
)