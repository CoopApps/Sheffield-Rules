import React from 'react';
import '../styles/confirmation-dialog.css';

interface ConfirmationDialogProps {
  isOpen: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  onConfirm: () => void;
  onCancel: () => void;
  danger?: boolean;
}

export const ConfirmationDialog: React.FC<ConfirmationDialogProps> = ({
  isOpen,
  title,
  message,
  confirmText = 'Confirm',
  cancelText = 'Cancel',
  onConfirm,
  onCancel,
  danger = false
}) => {
  if (!isOpen) return null;

  return (
    <div className="confirmation-overlay">
      <div className="confirmation-dialog">
        <div className="confirmation-header">
          <h2>{title}</h2>
        </div>
        <div className="confirmation-body">
          <p>{message}</p>
        </div>
        <div className="confirmation-footer">
          <button
            className="confirmation-button cancel"
            onClick={onCancel}
          >
            {cancelText}
          </button>
          <button
            className={`confirmation-button confirm ${danger ? 'danger' : ''}`}
            onClick={onConfirm}
          >
            {confirmText}
          </button>
        </div>
      </div>
    </div>
  );
};

export default ConfirmationDialog;
