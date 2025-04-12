import { PrismaClient } from '@prisma/client';
import express from 'express';
import cors from 'cors';

const app = express();
const prisma = new PrismaClient();

app.use(cors()); // Enable CORS for frontend requests
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Route to submit new feedback
app.post('/api/feedback', async (req, res) => {
  try {
    const { title, description } = req.body;
    
    // Create feedback in database
    const feedback = await prisma.feedback.create({
      data: {
        title,
        description,
        timestamp: new Date()
      }
    });
    
    res.status(201).json({
      success: true,
      data: feedback
    });
  } catch (error) {
    console.error('Error submitting feedback:', error);
    res.status(500).json({
      success: false,
      message: 'Failed to submit feedback'
    });
  }
});

// Route to get all feedback
app.get('/api/feedback', async (req, res) => {
  try {
    const feedbacks = await prisma.feedback.findMany({
      orderBy: {
        timestamp: 'desc'
      }
    });
    
    res.json({
      success: true,
      data: feedbacks
    });
  } catch (error) {
    console.error('Error retrieving feedback:', error);
    res.status(500).json({
      success: false,
      message: 'Failed to retrieve feedback'
    });
  }
});

// Start server
const PORT = process.env.PORT || 3001;
app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});