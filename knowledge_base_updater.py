
# Import required libraries
import json
import os

class KnowledgeBaseUpdater:
    def __init__(self, knowledge_base_path):
        self.knowledge_base_path = knowledge_base_path

    def update_knowledge_base(self, new_data):
        # Check if the knowledge base file exists
        if os.path.exists(self.knowledge_base_path):
            # Load existing knowledge base data
            with open(self.knowledge_base_path, 'r') as file:
                existing_data = json.load(file)
        else:
            # Initialize an empty knowledge base if the file doesn't exist
            existing_data = {}

        # Update the knowledge base with new data
        existing_data.update(new_data)

        # Save the updated knowledge base
        with open(self.knowledge_base_path, 'w') as file:
            json.dump(existing_data, file, indent=4)
    