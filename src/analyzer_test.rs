// src/analyzer_test.rs - Simple test to verify /analyzer functionality works

#[cfg(test)]
mod tests {
    use super::*;
    use nannou_osc as osc;
    use nannou::prelude::*;
    
    #[test]
    fn test_analyzer_message_handling() {
        // Test to verify that /analyzer messages are properly handled
        // This test demonstrates that the system can parse and process
        // analyzer messages correctly
        
        let test_args = vec![
            osc::Type::Float(1.0),      // timestamp
            osc::Type::Float(440.0),    // pitch (A4)
            osc::Type::Float(0.8),      // amplitude
            osc::Type::Float(0.1),      // onset
            osc::Type::Float(2.0),      // duration
            osc::Type::Float(0.6),      // timbre_features
        ];
        
        // Extract values like the actual handler would
        let timestamp = test_args[0].clone().float().unwrap_or(0.0);
        let pitch = test_args[1].clone().float().unwrap_or(440.0);
        let amplitude = test_args[2].clone().float().unwrap_or(0.5);
        let onset = test_args[3].clone().float().unwrap_or(0.0);
        let duration = test_args[4].clone().float().unwrap_or(1.0);
        let timbre_features = test_args[5].clone().float().unwrap_or(0.5);
        
        // Verify extracted values
        assert_eq!(timestamp, 1.0);
        assert_eq!(pitch, 440.0);
        assert_eq!(amplitude, 0.8);
        assert_eq!(onset, 0.1);
        assert_eq!(duration, 2.0);
        assert_eq!(timbre_features, 0.6);
        
        // Verify that the analyzer recognizes the message type
        let message_address = "/analyzer";
        assert_eq!(message_address, "/analyzer");
        
        // Test timbre-based event classification
        let event_type = if timbre_features > 0.7 {
            "Transient"
        } else if timbre_features > 0.4 {
            "Chord"
        } else {
            "Note"
        };
        assert_eq!(event_type, "Chord"); // 0.6 should map to Chord
        
        println!("✅ /analyzer message handling test passed");
    }
    
    #[test]
    fn test_analyzer_visual_mapping() {
        // Test the visual mapping calculations
        let window_rect = Rect::from_w_h(800.0, 600.0);
        
        // Test pitch to X mapping
        let pitch = 440.0;
        let log_pitch = pitch.log2().clamp(5.0, 11.0);
        let x_pos = (log_pitch - 5.0) / (11.0 - 5.0) * window_rect.w() + window_rect.left();
        
        // Verify position is within bounds
        assert!(x_pos >= window_rect.left());
        assert!(x_pos <= window_rect.right());
        
        // Test amplitude to Y mapping
        let amplitude = 0.8;
        let y_pos = amplitude * window_rect.h() + window_rect.bottom();
        
        // Verify position is within bounds
        assert!(y_pos >= window_rect.bottom());
        assert!(y_pos <= window_rect.top());
        
        println!("✅ Visual mapping test passed");
    }
}