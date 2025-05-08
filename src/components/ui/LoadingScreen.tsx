import { motion } from 'framer-motion';
import { FlaskRound as Flask } from 'lucide-react';

export default function LoadingScreen() {
  return (
    <div className="flex items-center justify-center min-h-screen bg-gray-50">
      <div className="text-center">
        <motion.div
          animate={{ 
            scale: [1, 1.2, 1],
            rotate: [0, 10, -10, 0]
          }}
          transition={{ 
            duration: 2, 
            repeat: Infinity,
            repeatType: 'loop'
          }}
          className="inline-block text-primary-600 mb-4"
        >
          <Flask size={48} />
        </motion.div>
        <h2 className="text-xl font-semibold text-gray-900 mb-2">Loading...</h2>
        <p className="text-gray-500">Preparing laboratory interface</p>
      </div>
    </div>
  );
}