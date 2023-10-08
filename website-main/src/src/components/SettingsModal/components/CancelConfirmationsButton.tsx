import { language } from '../../../main'
import { Button, Spinner } from 'react-bootstrap'

interface CancelConfirmationsButtonProps {
  loading: boolean;
  onCancel: () => void;
}

const CancelConfirmationsButton = ({ loading, onCancel }: CancelConfirmationsButtonProps) => {

  return (
    <Button onClick={() => onCancel()} variant="outline-primary">
      {loading ?
        <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
        :
        language.dictionary.cancelPendingConfirmations
      }
    </Button>
  )
}

export default CancelConfirmationsButton